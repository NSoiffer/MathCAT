# convert CSV file (saved from Murray Sargent's word file) for Unicode -> Nemeth (and UEB) mappings
#  this file doesn't include digits and letters, so they are added separately
# FIX: this table seems incomplete when compared with SRE
# FIX: add SRE json contents from speech-rule-engine\mathmaps\nemeth when there is a translation
# FIX:   it would be good to add in the description, but that involves look up into the Unicode file
import csv
def write_nemeth_yaml(in_file, out_file):
    with open(out_file, 'w', encoding="utf8") as out_stream:
        with open(in_file, encoding="utf8") as csv_file:
            csv_reader = csv.reader(csv_file, delimiter=',')
            out_stream.write("---\n")
            write_letters_and_digits(out_stream)
            # entries are a list of numeric code point, char, full name, Nemeth, UEB
            for entry in csv_reader:
                # we do the write in two parts so that the comment is aligned
                first_part = ' - "{}": [t: {}]'.format(unicode_char(entry), nemeth(entry))
                out_stream.write('{:32}# 0x{} ({})\n'.format(
                        first_part, code_point(entry), unicode_name(entry)))

def code_point(list):
    return list[0]

def unicode_char(list):
    ch = list[1]
    # escape quotes and backslashes
    if (ch == '"' or ch == '\\'):
        ch = "\\" + ch
    return ch

def unicode_name(list):
    return list[2]

def nemeth(list):
    return list[3]

def ueb(list):
    return list[4]

def write_letters_and_digits(out_stream):
    digits = ["⠴", "⠂","⠆","⠒","⠲","⠢","⠖","⠶","⠦","⠔"]
    small_latin = ["⠰⠁", "⠰⠃", "⠰⠉", "⠰⠙", "⠰⠑", "⠰⠋", "⠰⠛", "⠰⠓", "⠰⠊", "⠰⠚", "⠰⠅", "⠰⠇", "⠰⠍",
                 "⠰⠝", "⠰⠕", "⠰⠏", "⠰⠟", "⠰⠗", "⠰⠎", "⠰⠞", "⠰⠥", "⠰⠧", "⠰⠺", "⠰⠭", "⠰⠽", "⠰⠵" ]
    cap_latin = ["⠰⠠⠁", "⠰⠠⠃", "⠰⠠⠉", "⠰⠠⠙", "⠰⠠⠑", "⠰⠠⠋", "⠰⠠⠛", "⠰⠠⠓", "⠰⠠⠊", "⠰⠠⠚", "⠰⠠⠅",
                 "⠰⠠⠇", "⠰⠠⠍", "⠰⠠⠝", "⠰⠠⠕", "⠰⠠⠏", "⠰⠠⠟", "⠰⠠⠗", "⠰⠠⠎", "⠰⠠⠞", "⠰⠠⠥", "⠰⠠⠧", "⠰⠠⠺", "⠰⠠⠭", "⠰⠠⠽", "⠰⠠⠵" ]
    write_range(out_stream, digits, '0')
    write_range(out_stream, small_latin, 'a')
    write_range(out_stream, cap_latin, 'A')


def write_range(out_stream, list, first_char):
    for i in range(0,len(list)):
        unicode = ord(first_char) + i
        first_part = ' - "{}": [t: {}]'.format(chr(unicode), list[i])
        out_stream.write('{:32}# 0x{}\n'.format(first_part, unicode))


write_nemeth_yaml("braille.csv", "unicode.yaml")