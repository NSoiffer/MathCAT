from ascii_braille import ascii_to_euro_braille
from bs4 import BeautifulSoup
from html_table_extractor.extractor import Extractor
from typing import TextIO
import sys
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
    if ch == '"':
        ch = '\\"'
    elif ch == '\\':
        ch = '\\\\'
    elif ch == '\\127':
        ch = '\\x7F'
    braille = ascii_to_euro_braille(latex + ' ')
    first_part = f' - "{ch}": [t: "{braille}"]'
    try:
        comment = ''
        if ch == '\\\\' or ch == '\\"':
            comment = hex(ord(ch[1]))
        elif len(ch) == 1 or len(ch) == 2:
            comment = hex(ord(ch))
        else:
            comment = "0" + ch[1:]
        out_stream.write('{:40}# {} ({})\n'.format(first_part, comment, latex))
    except:
        print(f"failed to write a line for ch='{ch}'")


# create_unicode_from_list_of_symbols_html("euro-symbols2.yaml")
create_greek_letters("greek-letters.yaml")
