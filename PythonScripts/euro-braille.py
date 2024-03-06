from ascii_braille import ascii_to_euro_braille
from bs4 import BeautifulSoup
from html_table_extractor.extractor import Extractor
from typing import TextIO
import sys
from string import ascii_uppercase, ascii_lowercase
import xml.etree.ElementTree as ET
import re
sys.stdout.reconfigure(encoding='utf-8')


def create_unicode_from_latex_symbols_html(out_file: str):
    with open("latex-symbols.htm", encoding='utf8') as in_stream:
        with open(out_file, 'w', encoding='utf8') as out_stream:
            file_contents = BeautifulSoup(in_stream, features="html.parser")
            def_list = file_contents.select_one('dl')
            if def_list is None:
                print("didn't find 'dl'")
                return
            latex_list = list(map(lambda x: x.contents[0], def_list.select('dt code')))
            # unicode_list = list(map(lambda x: x.find('p').contents[0].split(' ')[0], def_list.select('dd')))
            foo = def_list.select('dd')
            unicode_list = list(map(lambda x: x.find('p').contents[0].split(' ')[0], foo))
            combined = sorted(zip(unicode_list, latex_list))
            for unicode, latex in combined:
                write_line(unicode, latex, out_stream)


def create_unicode_from_list_of_symbols_html(out_file: str):
    # the HTML file has rowspans in it -- hence the use of table extractor
    with open("List of mathematical symbols by subject.htm", encoding='utf8') as in_stream:
        with open(out_file, 'w', encoding='utf8') as out_stream:
            file_contents = BeautifulSoup(in_stream, features="html.parser")
            tables = file_contents.select('table')
            if tables is None:
                print("didn't find 'tables'")
                return
            all_entries = []
            i = 0
            for table in tables:
                # print(f"table {i}")
                table_string = table.decode()
                extractor = Extractor(table_string)
                extractor.parse()
                rows = extractor.return_list()
                i_latex = 3 if len(rows[1]) == 6 else 4
                # print(f"row 2='{rows[2]}'")
                for row in rows[1:]:
                    try:
                        unicode = row[1].strip()
                    except:
                        print(f"Error in getting unicode in {row}")
                    if len(unicode) == 1:       # filter out "det", etc
                        latex: str = row[i_latex].split(',')[0].strip()
                        if latex.startswith('\\'): # filter out ASCII and some other oddballs
                            # print(f"unicode={unicode}, latex={latex}")
                            all_entries.append((unicode, latex))
                i += 1
            all_entries = sorted(list(dict.fromkeys(all_entries)))
            for unicode, latex in all_entries:
                write_line(unicode, latex, out_stream)


UNICODE_CH_PATTERN = re.compile(r' - "(.)"')


def get_unicode_yaml_chars() -> set[str]:
    answer = set()
    with open("../Rules/Languages/en/unicode.yaml", "r", encoding='utf8') as unicode_stream:
        for line in unicode_stream.readlines():
            matched = UNICODE_CH_PATTERN.match(line)
            if matched and ord(matched.group(1)) > 127:
                answer.add(matched.group(1))
        for ch in range(ord('Α'), ord('Ω')):   # these are a range in unicode.yaml, so the pattern doesn't match
            answer.add(chr(ch))
    return answer


# The chars in unicode.yaml (others go into unicode-full.yaml)
UNICODE_CHARS_SHORT = get_unicode_yaml_chars()


def get_short_dict() -> dict[str, str]:
    with open("euro-braille-short.csv", "r", encoding='utf8') as stream:
        answer = {}
        for line in stream.readlines():
            parts: list[str] = list(filter(lambda x: x != '', line.split(' ')))
            short_name = parts[0].strip()
            latex_name = parts[1].strip()
            answer[latex_name] = short_name
        for ch in ascii_lowercase + ascii_uppercase:
            answer[f'\\mathbb{{{ch}}}'] = f'\\{ch}'
        return answer


def extract_latex(in_file):
    short_names = get_short_dict()
    overrides = {
        "*": "*", "{": "\\{", "}": "\\}", "|": "|",
        "°": "°", "ϵ": "\\epsilon", "≠": "\\not=",   # varepsilon
        "′": "'", "″": "''", "‴": "'''",
        "△": "\\triangle", "→": "\\to",
    }

    tree = ET.parse(in_file)
    root = tree.getroot()
    all_chars = root.find("charlist")

    with open("latex-braille-unicode.yaml", 'w', encoding='utf8') as short_stream:
        with open("latex-braille-unicode-full.yaml", 'w', encoding='utf8') as full_stream:
            short_stream.write("---\n")
            full_stream.write("---\n")
            for char in all_chars:
                ch = convert_to_char(char.get("id"))
                if len(ch) > 1:
                    continue
                code = ord(ch)
                if code < 0x20:
                    continue
                if ch in overrides:
                    latex_name = overrides[ch]
                    write_line(ch, latex_name, short_names.get(latex_name, ''), short_stream)
                    continue

                # add in ASCII and the Greek block
                stream = short_stream if ch in UNICODE_CHARS_SHORT or code < 0x7F or (0x0370 <= code and code <= 0x03fF) else full_stream

                # I wish there was a simple way to choose the names.
                # Based on what David Carlisle (who maintains unicode.xml) recomends,
                #   'math_latex' is the preferred field except for the alphabets (I only exclude Greek and math alphanumerics)
                #   For those, math_latex is more technically correct but not what most latex users are accustomed to
                names_seen: list[str] = []
                for style in ["mathlatex", "latex", "varlatex", "ams"]:
                    latex_name = char.find(style)
                    if latex_name is None:
                        continue
                    latex_name:str = latex_name.text.strip()
                    # the fontencoding char won't happen and the \unicode (two ellipsis entries) have short names for the latex style
                    if latex_name.startswith('{\\fontencoding{') or latex_name.startswith('\\unicode'):
                        continue
                    if not latex_name.startswith('\\') and not latex_name.startswith('{') and code >= 0x7F:
                        latex_name = '\\' + latex_name  # some are missing the initial \
                    if latex_name.startswith('\\mathchar'):
                        continue    # seems to happen once -- not sure what that is about
                    if style == 'mathlatex':
                        if code < 0x7F:
                            continue    # use the latex names
                        if 0x0370 <= code and code <= 0x03fF:
                            continue    # Greek block
                        if 0x1D400 <= code and code <= 0x1D7FF:
                            continue    # alphanumerics
                        if latex_name.startswith('\\Bbb'):      # some blackboard chars (ℝ, etc) not in math alphanumerics
                            continue
                        if latex_name.startswith('\\mbox'):
                            continue    # the alternative name avoids that and so is better
                    if latex_name.lower().find('theta') != -1:
                        latex_name = latex_name.replace("text", "")  # don't care about upright theta
                    elif ch == '$':
                        latex_name = '\\$'
                    elif ch == '\\':
                        latex_name = '\\backslash'  # avoid '\textbackslash'
                    elif latex_name.startswith("\\mitBbb"):
                        latex_name = latex_name.replace("\\mitBbb", "")     # exponential e, etc
                    if latex_name in names_seen:
                        continue
                    if len(names_seen) > 0:
                        stream.write('# ')    # alternative name
                    write_line(ch, latex_name, short_names.get(latex_name, ''), stream)
                    names_seen.append(latex_name)

            # write the invisible chars out
            short_stream.write('\n # invisible chars\n')
            write_line(chr(0x2061), '', '', short_stream)
            write_line(chr(0x2062), '', '', short_stream)
            write_line(chr(0x2063), '', '', short_stream)
            write_line(chr(0x2064), '', '', short_stream)


def convert_to_char(str: str) -> str:
    # str is 'Uddddd' or 'Uddddd-ddddd'
    str = str.split("U")[1]  # strip leading 'U'
    answer = ""
    for char_str in str.split("-"):
        # FIX: need to add backslash is str becomes ""
        ch = chr(int(char_str, base=16))
        # if (ch == '"' or ch == '\\'):
        #     answer += "\\"
        answer += ch

    return answer


def write_line(ch: str, latex: str, short: str, out_stream: TextIO):
    def hex_string(ch: str) -> str:
        comment = ''
        if ch == '\\\\' or ch == '\\"':
            comment = hex(ord(ch[1]))
        elif len(ch) == 1:
            comment = hex(ord(ch))
        else:
            comment = "0" + ch[1:]
        return comment
    
    if ord(ch) < 0x7F and len(latex) <= 1:
        return        # probably an ASCII char

    if ch == '"':
        ch = '\\"'
    elif ch == '\\':
        ch = '\\\\'
    elif ch == '\\127':
        ch = '\\x7F'
    elif ch == "°":
        latex = "°"     # special case in their code
    short_space = '𝐖' if short.startswith('\\') and not short.endswith('}') and len(short) > 2 else ''
    long_space = '𝐖' if latex.startswith('\\') and not latex.endswith('}') and len(latex) > 2 else ''
    try:
        # write untranslated text
        latex = latex.replace('\\', '\\\\').replace('"', '\\"')
        short = short.replace('\\', '\\\\').replace('"', '\\"')
        if short == '':
            first_part_char = f' - "{ch}": [t: "{latex + long_space}"]'
            out_stream.write(f'{first_part_char:<40} # {hex_string(ch)}\n')
        else:
            first_part_char = f' - "{ch}":'
            first_part_short = f'         then: [t: "{short + short_space}"]'
            first_part_long = f'         else: [t: "{latex + long_space}"]'
            out_stream.write(f'{first_part_char:<40} # {hex_string(ch)}\n')
            out_stream.write('     - test:\n')
            out_stream.write('         if: "$LaTeX_UseShortName"\n')
            out_stream.write(f'{first_part_short}\n')
            out_stream.write(f'{first_part_long}\n')  # not sure why, but this gives better alignment
        # write the translated dots
        # braille = ascii_to_euro_braille(latex + space)
        # if short == '':
        #     first_part_char = f' - "{ch}": [t: "{braille}"]'
        #     out_stream.write(f'{first_part_char:<40} # {hex_string(ch)} ({latex})\n')
        # else:
        #     short_braille = ascii_to_euro_braille(short+space)  # fix spacing
        #     first_part_char = f' - "{ch}":'
        #     first_part_short = f'         else: [t: "{short_braille}"]'
        #     first_part_long = f'         then: [t: "{braille}"]'
        #     out_stream.write(f'{first_part_char:<40} # {hex_string(ch)}\n')
        #     out_stream.write('     - test:\n')
        #     out_stream.write('         if: "$LaTeX_UseShortName=\'True\'"\n')
        #     out_stream.write(f'{first_part_long:<34} # {latex}\n')  # not sure why, but this gives better alignment
        #     out_stream.write(f'{first_part_short:<36} # {short}\n')
    except:
        print(f"failed to write a line for ch='{ch}/{hex_string(ch)}'")


def create_greek_letters(out_file: str):
    # the HTML file has rowspans in it -- hence the use of table extractor
    with open("greek-letters.txt", encoding='utf8') as in_stream:
        with open(out_file, 'w', encoding='utf8') as out_stream:
            all_entries = []
            lines = in_stream.readlines()
            for line in lines:
                parts = line.split('\t')
                if parts[1].startswith('\\'):       # ignore 'A', etc., which don't have latex commands
                    all_entries.append((parts[0].strip(), parts[1].strip()))
            all_entries = sorted(all_entries)
            for unicode, latex in all_entries:
                write_line(unicode, latex, out_stream)




# create_unicode_from_list_of_symbols_html("euro-symbols2.yaml")
# create_greek_letters("greek-letters.yaml")
extract_latex("c:\\dev\\mathml-refresh\\xml-entities\\unicode.xml")
