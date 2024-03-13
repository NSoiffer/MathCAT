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
                write_line(unicode, latex, "", out_stream)


COMMENT = """\
# This file is derived from a number of sources.
# This tries to conform to the "spec" augenbit.de/wiki/index.php?title=LaTeX-Manual_LaTeX_Grundregeln (and linked files)
# The short names come from MathLib.tex that is linked from above
# Where there is not a conflict, the next source of names comes from "standard" LaTeX names listed in
#   en.wikipedia.org/wiki/List_of_mathematical_symbols_by_subject
# Otherwise they are based on the XML entities document (w3c.github.io/xml-entities).
#  That references the data file github.com/w3c/xml-entities/blob/gh-pages/unicode.xml
#  The fields in that are used are ["mathlatex", "latex", "varlatex", "ams"], with different names added as comments.
# Note: there is some filtering of unlikely names, so the above is not 100% as to the output, but it is close.
"""


def get_unicode_standard_symbols() -> dict[str, list[str]]:
    # the HTML file has rowspans in it -- hence the use of table extractor
    with open("List of mathematical symbols by subject.htm", encoding='utf8') as in_stream:
        file_contents = BeautifulSoup(in_stream, features="html.parser")
        tables = file_contents.select('table')
        if tables is None:
            print("didn't find 'tables'")
            return {}
        all_entries: list[tuple[str, str]] = []
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
                    unicode = row[1].text
                    if not isinstance(unicode, str):
                        continue
                    unicode = unicode.strip()
                    # print(f"unicode='{unicode}' latex='{row[i_latex]}', latex type={type(row[i_latex])}")
                    # print(f"  latex='{row[i_latex]}, type={type(row[i_latex])}")
                except (IndexError, TypeError, AttributeError) as err:
                    print(f"Error in getting unicode in {row}\nError is '{err}'")
                if len(unicode) == 1:                          # filter out "det", etc
                    latex_col = row[i_latex]
                    for code in latex_col.select('code'):
                        # print(f"  *** code='{code.text}', type={type(code.text)}")
                        latex: str = code.text.strip()
                        if latex.startswith('\\'):
                            # filter out ASCII and some other oddballs
                            all_entries.append((unicode, latex))
            i += 1
        print(f'all_entries len={len(all_entries)}')
        answer: dict[str, list[str]] = {}
        for (key, val) in all_entries:
            if key in answer:
                if val not in answer[key]:
                    answer[key].append(val)
            else:
                answer[key] = [val]
        print(f'answer len={len(list(answer))}')
        return answer


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
            answer[f'\\mathbb𝐖{ch}'] = f'\\{ch}'
        return answer


CHAR_IN_BRACES = re.compile(r'(\\.+)\{(.)\}')


def extract_latex(in_file):
    short_names = get_short_dict()
    standard_names = get_unicode_standard_symbols()
    print(f'len standard names = {len(standard_names)}')
    overrides = {
        "*": "*", "{": "\\{", "}": "\\}", "|": "|", "%": "%",
        "°": "°", "ϵ": "\\epsilon", "≠": "\\not=",   # varepsilon
        "′": "'", "″": "''", "‴": "'''",
        "≤": "\\le", "≥": "\\ge",
        "\\cdotp": "\\cdots", "\\varnothing": "\\emptyset",
        "△": "\\triangle", "→": "\\to",
        "‰": "\\permil",
    }

    tree = ET.parse(in_file)
    root: ET.Element = tree.getroot()
    all_char_elements = root.find("charlist")
    if all_char_elements is None:
        print(f"Didn't find XML root in {in_file}!")
        exit(1)

    with open("latex-braille-unicode.yaml", 'w', encoding='utf8') as short_stream:
        with open("latex-braille-unicode-full.yaml", 'w', encoding='utf8') as full_stream:
            short_stream.write(COMMENT)
            full_stream.write(COMMENT)
            short_stream.write("\n---\n")
            full_stream.write("\n---\n")
            for char_element in all_char_elements:
                if char_element is None:
                    print("char_element is None!")
                    continue
                ch = char_element.get("id")
                if ch is None:
                    print('char_element.get("id") is None!')
                    continue
                ch = convert_to_char(ch)
                if len(ch) > 1:
                    continue
                code = ord(ch)
                if code < 0x20:
                    continue
                # add in ASCII and the Greek block
                is_in_common_char_blocks = code < 0x7F or (0x0370 <= code and code <= 0x03fF)
                stream = short_stream if ch in UNICODE_CHARS_SHORT or is_in_common_char_blocks else full_stream
                # use the standard name unless the char is in the override dict (if it and the standard name is an option, write it first)
                if ch in standard_names:
                    latex_names = standard_names[ch]
                    is_overridden = ch in overrides
                    first_time = True
                    if is_overridden:
                        latex_name = overrides[ch]
                        write_line(ch, latex_name, short_names.get(latex_name, ''), False, stream)
                        first_time = False
                    # print(f"standard name list for {ch}: {latex_names}")
                    for latex_name in latex_names:
                        is_commented = False
                        if is_overridden and latex_name == overrides[ch]:
                            continue
                        if first_time:
                            first_time = False
                        else:
                            is_commented = True    # alternative name
                        if CHAR_IN_BRACES.search(latex_name):   # probably a better way to do this
                            latex_name = CHAR_IN_BRACES.sub(lambda match: f'{match.group(1)}𝐖{match.group(2)}', latex_name,)
                        write_line(ch, latex_name, short_names.get(latex_name, ''), is_commented, stream)
                    continue
                if ch in overrides:
                    print(f"found override for '{ch}': {overrides[ch]}")
                    latex_name = overrides[ch]
                    write_line(ch, latex_name, short_names.get(latex_name, ''), False, stream)
                    continue


                # I wish there was a simple way to choose the names.
                # Based on what David Carlisle (who maintains unicode.xml) recomends,
                #   'math_latex' is the preferred field except for the alphabets (I only exclude Greek and math alphanumerics)
                #   For those, math_latex is more technically correct but not what most latex users are accustomed to
                names_seen: list[str] = []
                for style in ["mathlatex", "latex", "varlatex", "ams"]:
                    latex_name = char_element.find(style)
                    if latex_name is None:
                        continue
                    latex_name = latex_name.text
                    if latex_name is None:
                        continue
                    latex_name = latex_name.strip()
                    # the fontencoding char won't happen
                    # the \unicode (two ellipsis entries) have short names for the latex style
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
                    elif CHAR_IN_BRACES.search(latex_name):   # probably a better way to do this
                        latex_name = CHAR_IN_BRACES.sub(lambda match: f'{match.group(1)}𝐖{match.group(2)}', latex_name,)
                    if latex_name in names_seen:
                        continue
                    is_commented = False
                    if len(names_seen) > 0:
                        is_commented = True    # alternative name
                    write_line(ch, latex_name, short_names.get(latex_name, ''), is_commented, stream)
                    names_seen.append(latex_name)

            # write the invisible chars out
            short_stream.write('\n # invisible chars\n')
            write_line(chr(0x2061), '', '', False, short_stream)
            write_line(chr(0x2062), '', '', False, short_stream)
            write_line(chr(0x2063), '', '', False,  short_stream)
            write_line(chr(0x2064), '', '', False, short_stream)


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


def write_line(ch: str, latex: str, short: str, is_commented: bool, out_stream: TextIO):
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
        comment = "#" if is_commented else ""
        if short == '':
            first_part_char = f'{comment} - "{ch}": [t: "{latex + long_space}"]'
            out_stream.write(f'{first_part_char:<40} # {hex_string(ch)}\n')
        else:
            first_part_char = f'{comment} - "{ch}":'
            first_part_short = f'{comment}         then: [t: "{short + short_space}"]'
            first_part_long = f'{comment}         else: [t: "{latex + long_space}"]'
            out_stream.write(f'{first_part_char:<40} # {hex_string(ch)}\n')
            out_stream.write(f'{comment}     - test:\n')
            out_stream.write(f'{comment}         if: "$LaTeX_UseShortName"\n')
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
    except IOError:
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
                write_line(unicode, latex, "", False, out_stream)


# create_unicode_from_list_of_symbols_html("euro-symbols2.yaml")
# create_greek_letters("greek-letters.yaml")
extract_latex("c:\\dev\\mathml-refresh\\xml-entities\\unicode.xml")
