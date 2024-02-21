from ascii_braille import ascii_to_euro_braille
from bs4 import BeautifulSoup
from typing import TextIO
import sys
sys.stdout.reconfigure(encoding='utf-8')


def create_unicode_from_html(in_file: str, out_file):
    with open(in_file, encoding='utf8') as in_stream:
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
        out_stream.write('{:32}# {}\n'.format(first_part, comment))
    except:
        print(f"failed to write a line for ch='{ch}'")


create_unicode_from_html("latex-symbols.htm", "euro-symbols.yaml")
