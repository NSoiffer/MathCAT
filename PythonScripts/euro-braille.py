from ascii_braille import ascii_to_euro_braille
from bs4 import BeautifulSoup
from html_table_extractor.extractor import Extractor
from typing import TextIO
import sys
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


def extract_latex(in_file):
    tree = ET.parse(in_file)
    root = tree.getroot()
    all_chars = root.find("charlist")

    with open("latex-braille-unicode.yaml", 'w', encoding='utf8') as short_stream:
        with open("latex-braille-unicode-full.yaml", 'w', encoding='utf8') as full_stream:
            short_stream.write("---\n")
            full_stream.write("---\n")
            for char in all_chars:
                ch = convert_to_char(char.get("id"))
                if len(ch) == 1 and ord(ch) < 128:
                    continue

                stream = short_stream if ch in UNICODE_CHARS_SHORT else full_stream
                latex = char.find("latex")
                var_latex = char.find("varlatex")
                ams_latex = char.find("ams")
                math_latex = char.find("mathlatex")
                # if latex is None and not(var_latex is None and math_latex is None):
                #     print(f"No latex for ch: {ch}/{char.get('id')}" +
                #         "" if var_latex is None else f"var_latex={var_latex.text}" +
                #         "" if math_latex is None else f"math_latex={math_latex.text}"
                #     )
                #     continue

                names_seen = []
                for latex_name in [latex, var_latex, ams_latex, math_latex]:
                    if latex_name is None:
                        continue
                    latex_name = latex_name.text.strip()
                    if latex_name in names_seen:
                        continue
                    if latex_name.startswith('\\up') and "\\" + latex_name[3:] in names_seen:  # "\upiota", etc, is skipped
                        continue
                    if len(names_seen) > 0:
                        stream.write('# ')    # alternative name
                    write_line(ch, latex_name, stream)
                    names_seen.append(latex_name)


def convert_to_char(str: str) -> str:
    # str is 'Uddddd' or 'Uddddd-ddddd'
    str = str.split("U")[1]  # strip leading 'U'
    answer = ""
    for char_str in str.split("-"):
        # FIX: need to add backslash is str becomes ""
        ch = chr(int(char_str, base=16))
        if (ch == '"' or ch == '\\'):
            answer += "\\"
        answer += ch

    return answer


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


def write_line(ch: str, latex: str, out_stream: TextIO):
    def hex_string(ch: str) -> str:
        comment = ''
        if ch == '\\\\' or ch == '\\"':
            comment = hex(ord(ch[1]))
        elif len(ch) == 1:
            comment = hex(ord(ch))
        else:
            comment = "0" + ch[1:]
        return comment
    
    if ch == '"':
        ch = '\\"'
    elif ch == '\\':
        ch = '\\\\'
    elif ch == '\\127':
        ch = '\\x7F'
    space = '' if ch.startswith('\\') and not(ch.endswith('}')) else ' '
    braille = ascii_to_euro_braille(latex + space)
    first_part = f' - "{ch}": [t: "{braille}"]'
    try:
        out_stream.write('{:40}# {} ({})\n'.format(first_part, hex_string(ch), latex))
    except:
        print(f"failed to write a line for ch='{ch}/{hex_string(ch)}'")



# create_unicode_from_list_of_symbols_html("euro-symbols2.yaml")
# create_greek_letters("greek-letters.yaml")
extract_latex("c:\\dev\\mathml-refresh\\xml-entities\\unicode.xml")
