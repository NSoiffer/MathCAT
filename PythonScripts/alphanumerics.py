# Nemeth alphanumeric unicode converter
from bs4 import BeautifulSoup

def create_unicode_from_html(in_file: str, out_file):
    with open(in_file, encoding='utf8') as in_stream:
        with open(out_file, 'a', encoding='utf8') as out_stream:
            file_contents = BeautifulSoup(in_stream, features="html.parser")
            for row in file_contents.find_all('tr'):
                cols = row.find_all('td')
                if len(cols) > 0:
                    generate_char(out_stream, cols[1].get_text(), cols[2].get_text(), cols[3].get_text())

def generate_char(out_stream, hex: str, description: str, braille: str):
    #  - "⬟": [t: "⠫⠸⠢"]              # 0x2B1F (Black pentagon)
    first_part = ' - "{}": [t: "{}"]'.format(chr(int(hex, base=16)), braille)
    out_stream.write('{:32}# 0x{} ({})\n'.format(
            first_part, hex, description))

def generate_digits(out_file, prefix: str, hex: str, description: str):
    with open(out_file, 'a', encoding='utf8') as out_stream:
        i = 0
        for digit in "⠴⠂⠆⠒⠲⠢⠖⠶⠦⠔":
            as_int = int(hex,16) +i
            generate_char(out_stream, "{0:x}".format(as_int), "{} {}".format('bold', i), prefix+'⠼'+digit)
            i += 1



# get the JSON versions -- the HTML versions seem wrong in many places (e.g., bold or numbers)
import json
def create_unicode_from_json(in_file: str, out_file):
    with open(in_file, encoding='utf8') as in_stream:
        with open(out_file, 'a', encoding='utf8') as out_stream:
            file_contents = json.load(in_stream)
            info = file_contents["information"].replace("tests", "chars")
            out_stream.write('\n  # --- {} ---\n'.format(info))
            for char, braille in file_contents['tests'].items():
                if braille["expected"] != "":
                    generate_char_line(out_stream, char, braille["expected"], "default_alphabet" in in_file)

import re
MATCH_FACE_LANG_CHAR = re.compile(
    ("("
       "(?P<prefix>^)"
       "(?P<sans>⠠⠨)?"
       "(?P<bold>⠸(?=[^⠠⠔⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚⠅⠇⠍⠝⠕⠏⠟⠗⠎⠞⠥⠧⠺⠭⠽⠵⠯⠫⠱⠹]))?"
       "(?P<script>⠈(?=[^⠠⠔⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚⠅⠇⠍⠝⠕⠏⠟⠗⠎⠞⠥⠧⠺⠭⠽⠵⠯⠫⠱⠹]))?"
       "(?P<italic>⠨(?=[^⠠⠔⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚⠅⠇⠍⠝⠕⠏⠟⠗⠎⠞⠥⠧⠺⠭⠽⠵⠯⠫⠱⠹]))?"
       "((((?P<en>⠰)|(?P<de>⠸)|(?P<el>⠨)|(?P<el_var>⠨⠈))?"
       "(?P<cap>⠠)?(?P<char>[⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚⠅⠇⠍⠝⠕⠏⠟⠗⠎⠞⠥⠧⠺⠭⠽⠵⠯⠫⠱⠹]))|"
       "((?P<num>⠼?)(?P<digit>[⠴⠂⠆⠒⠲⠢⠖⠶⠦⠔])))"
       "(?P<postfix>$)"
      ")")
)

# fix some translations that SRE has wrong
SRE_CHAR_FIXES = {
    "⪼": "⠨⠨⠂⠈⠨⠨⠂⠻",
    "≻": "⠨⠨⠂", 
    "≿": "⠨⠨⠂⠈⠱",
    "⪺": "⠨⠨⠂⠌⠈⠱⠈⠱",
    "⪸": "⠨⠨⠂⠈⠱⠈⠱",
    "⪶": "⠨⠨⠂⠌⠨⠅",
    "⪴": "⠨⠨⠂⠨⠅",
    "⪲": "⠨⠨⠂⠌⠱",
    "⪰": "⠨⠨⠂⠱",
}

def generate_char_line(out_stream, char: str, braille: str, is_letter: bool):
    # format to generate
    #  - "⬟": [t: "⠫⠸⠢"]              # 0x2B1F

    if braille == "⠀":
        braille = "W"
    braille = SRE_CHAR_FIXES.get(char, braille)
    # escape quotes and backslashes
    if (char == '"' or char == '\\'):
        char = "\\" + char

    # pick the braille apart, looking for one or more typefaces, alphabet, (cap? letter) | (numeric? digit) | char
    
    result = ""
    matched = MATCH_FACE_LANG_CHAR.match(braille)

    if is_letter and matched:
        dict = matched.groupdict()
        if dict["prefix"]:
            result += dict["prefix"]
        if dict["sans"]:
            result += "S"
        if dict["bold"]:
            result += "B"
        if dict["script"]:
            result += "T"
        if dict["italic"]:
            result += "I"
        if dict["en"]:
            result += "E"
        if dict["de"]:
            result += "D"
        if dict["el"]:
            result += "G"
        if dict["el_var"]:
            result += "V"
        if dict["char"]:
            if dict["cap"]:
                result += "C"
            result += "L" + dict["char"]
        if dict["digit"]:
            result += "N" + dict["digit"]
        if dict["postfix"]:
            result += dict["postfix"]
    else:
        result = braille
    first_part = ' - "{}": [t: "{}"]'.format(char, result)
    out_stream.write('{:32}# {}\n'.format(first_part, hex(ord(char[-1])) ))

RUSSIAN_NEMETH = ['⠁', '⠃', '⠺', '⠛', '⠙', '⠑', '⠚', '⠵', '⠊', '⠅', '⠇', '⠍', '⠝', '⠕', '⠏',
                  '⠗', '⠎', '⠞', '⠥', '⠋', '⠓', '⠉', '⠟', '⠱', '⠭', '⠮', '⠪', '⠳', '⠫']

def generate_russian(out_stream):
    for i in range(0,len(RUSSIAN_NEMETH)):
        i_unicode = i if i < 9 else i+1   # need to skip a char
        if i_unicode > 25:              # need to skip another char or two
            if i_unicode > 26:
                i_unicode += 2
            else:
                i_unicode += 1
        lower = 0x0430 + i_unicode
        upper = 0x0410 + i_unicode
        first_part = ' - "{}": [t: "{}"]'.format(chr(lower), "U"+RUSSIAN_NEMETH[i])
        out_stream.write('{:32}# {}\n'.format(first_part, hex(lower)))
        first_part = ' - "{}": [t: "{}"]'.format(chr(upper), "UC"+RUSSIAN_NEMETH[i])
        out_stream.write('{:32}# {}\n'.format(first_part, hex(upper)))


                   

import os
# if os.path.exists("alphanumerics.txt"):
#   os.remove("alphanumerics.txt")
# path = "C:/Dev/speech-rule-engine/sre-tests/output/nemeth/symbols/"
# for filename in os.listdir(path):
#    create_unicode_from_html(path+filename, "alphanumerics.txt")
if os.path.exists("out"):
  os.remove("out")

# path = "C:/Dev/speech-rule-engine/sre-tests/expected/nemeth/symbols/"
# # file = "default_alphabet_bold.json"
# if os.path.exists("alphanumerics.txt"):
#   os.remove("alphanumerics.txt")
# for filename in os.listdir(path):
#     # these have multi char entries and aren't character definitions
#     if not(filename == 'default_functions.json' or filename == 'default_si_units.json' or filename == 'default_units.json'):
#         create_unicode_from_json(path+filename, "alphanumerics.txt")

with open("out", 'a', encoding='utf8') as out_stream:
    generate_russian(out_stream)
# generate_digits("out", "⠈", "1D7D8", "double struck (as script)")
# generate_digits("out", "⠠", "1D7E2", "sans-serif")
