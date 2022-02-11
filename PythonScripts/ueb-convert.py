# convert CSV file for Unicode -> UEB mappings
#  this file doesn't include digits and letters, so they are added separately
import csv
from ascii_braille import ascii_to_unicode

def write_ueb_yaml(in_file, out_file):
    with open(out_file, 'w', encoding="utf8") as out_stream:
        with open(in_file, encoding="utf-8-sig") as csv_file:
            csv_reader = csv.reader(csv_file, delimiter=',')
            out_stream.write("---\n")
            write_letters_and_digits(out_stream)
            # entries are a list of numeric code point, char, full name, Nemeth, UEB
            # entries are a list of char, ueb, hex unicode, full name
            for entry in csv_reader:
                # if ord(unicode_char(entry)) != code_point(entry):
                #     print("mismatch of code point and hex: %s" % entry)
                if unicode_char(entry) != "":   # some multiline UEB indicators
                    write_yaml_line(out_stream, unicode_char(entry), ueb(entry), code_point(entry), unicode_name(entry))

            # add invisible chars inserted by canonicalization
            write_special_lines(out_stream)
            out_stream.write("""
 - "⁡":                          # 0x2061⁡ (invisible function apply)
    - test:
        if: "following-sibling::*[1][self::m:mi and translate(., 'abcdefghijklmnopqrstuvwxyz', '') = '']" # GTM 9.3.2
        then: [t: "W"]
        else: [t: ""]
""")
            out_stream.write("""
 - "⁢":                         # 0x2062 (invisible times)
    - test:
        if: # GTM 9.3.3 (not very clear in rule, but the function name has to start with a lower case latin char ['no indicators'])
        - "parent::m:mrow and "
        - "preceding-sibling::*[1][self::m:mi and translate(., 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ', '') = ''] and"
        - "  following::*[1][self::m:mrow and count(*)=3 and "   # look for function apply
        - "     *[2][text()='\u2061'] and *[1][self::m:mi and translate(., 'abcdefghijklmnopqrstuvwxyz', '') = '']]" 
        then: [t: "W"]
        else: [t: ""]
""")

            write_yaml_line(out_stream, "⁣", "","2063⁡", "invisible separator")
            write_yaml_line(out_stream, "⁤", "","2064", "invisible plus")


def code_point(list):
    return list[2]

def unicode_char(list):
    ch = list[0]
    # escape quotes and backslashes
    if (ch == '"' or ch == '\\'):
        ch = "\\" + ch
    return ch

def unicode_name(list):
    return list[3]

def ueb(list):
    first_char = list[1][0]
    unicode = unicode_char(list)
    braille = ''
    # FIX: need to double check what other ascii chars need grade 1 indicators
    # if not((unicode.isascii() and unicode!=':') or (first_char in '#@^._";,') or (unicode in "…′″‴")): # list of UEB prefix chars -- no grade 1 indicator needed

    if not( (unicode.isascii() and not(unicode in ['?', '.', ',', ';', ':'])) or
            (first_char in '#@^._";,') or (unicode in "…′″‴") ): # list of UEB prefix chars -- no grade 1 indicator needed
        print("Adding grade 1 to: ", list)
        braille = '1'

    try:
        braille += ascii_to_unicode(list[1])
    except:
        print("Illegal ASCII braille: '{}'".format(list[1]))
    
    # shape terminator (check for shape, shaded shape, or bold shape)
    if first_char == '$' or list[1].startswith('.$') or list[1].startswith('_$'): 
        braille += 't'
    
    return braille


def write_yaml_line(out_stream, char, nemeth, hex, unicode_name):
    # we do the write in two parts so that the comment is aligned
    description = "" if unicode_name =="" else '(' + unicode_name + ')'
    first_part = ' - "{}": [t: "{}"]'.format(char, nemeth)
    out_stream.write('{:32}# 0x{} {}\n'.format(
            first_part, hex, description))


def write_letters_and_digits(out_stream):
    digits = ["⠚", "⠁", "⠃", "⠉", "⠙", "⠑", "⠋", "⠛", "⠓", "⠊"]

    # in UEB a, i, and o can never be confused with with grade 2 chars, although they need grade 1 symbols if in numeric mode
    small_latin = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m",
                   "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z" ]
    small_greek = ["a", "b", "g", "d", "e", "z", "⠱ ", "⠹", "i", "k", "l", "m", "n",
                   "x", "o", "p", "r", "s", "t", "u", "f", "⠯", "y", "w" ]
    write_letter_chars(out_stream, digits, '0', "N")

    write_letter_chars(out_stream, my_ascii_to_unicode(small_latin), 'a', "L")
    write_letter_chars(out_stream, my_ascii_to_unicode(small_latin), 'A', "L⠠")

    # no grade 1 indicator for Greek letters -- capitalization comes first
    write_letter_chars(out_stream, my_ascii_to_unicode(small_greek), 'α', "⠨")
    write_letter_chars(out_stream, my_ascii_to_unicode(small_greek), 'Α', "⠠⠨")   # capital before Greek
    write_special_greeks(out_stream)
    write_vulgar_fractions(out_stream)



def write_letter_chars(out_stream, list, first_char, prefix):
    for i in range(0,len(list)):
        unicode = ord(first_char) + i
        if list[i] != "":
            write_yaml_line(out_stream, chr(unicode), prefix + list[i], hex(unicode)[2:], "")
    out_stream.write("\n")

def my_ascii_to_unicode(list_of_chars):
    # converts lower case ASCII to unicode braille equivalents -- other chars (e.g, braille) is left unchanged
    result = []
    for ch in list_of_chars:
        if ch == "":
            result.append("")
        elif ch.islower():
            result.append(ascii_to_unicode(ch))
        else:
            result.append(ch)
    return result

def write_special_greeks(out_stream):
    # these have no consecutive sequence in UEB
    write_yaml_line(out_stream, "µ", "⠨"+ascii_to_unicode("m"),"00B5", "Micro (Greek mu)")
    write_yaml_line(out_stream, "Ω", "⠠⠨"+ascii_to_unicode("m"),"2126", "Ohm sign (capital Greek omega)")
    write_yaml_line(out_stream, "∆", "⠠⠨"+ascii_to_unicode("d"),"2206", "Increment (capital Greek delta)")
    write_yaml_line(out_stream, "∏", "⠠⠨"+ascii_to_unicode("p"),"220F", "Product (capital Greek pi)")
    write_yaml_line(out_stream, "∑", "⠠⠨"+ascii_to_unicode("s"),"2211", "Sum (capital Greek sigma)")
    out_stream.write("\n")

def  write_vulgar_fractions(out_stream):
    write_yaml_line(out_stream, "¼", "!N⠁N⠌N⠙","00BC", "Vulgar Fraction One Quarter")
    write_yaml_line(out_stream, "½", "!N⠁N⠌N⠃","00BD", "Vulgar Fraction One Half")
    write_yaml_line(out_stream, "¾", "!N⠉N⠌N⠙","00BE", "Vulgar Fraction Three Quarters")

    write_yaml_line(out_stream, "⅐", "!N⠁N⠌N⠛","2150", "Vulgar Fraction One Seventh")
    write_yaml_line(out_stream, "⅑", "!N⠁N⠌N⠊","2151", "Vulgar Fraction One Ninth")
    write_yaml_line(out_stream, "⅒", "!N⠁N⠌N⠁N⠚","2152", "Vulgar Fraction One Tenth")
    write_yaml_line(out_stream, "⅓", "!N⠁N⠌N⠉","2153", "Vulgar Fraction One Third")
    write_yaml_line(out_stream, "⅔", "!N⠃N⠌N⠉","2154", "Vulgar Fraction Two Thirds")
    write_yaml_line(out_stream, "⅕", "!N⠁N⠌N⠑","2155", "Vulgar Fraction One Fifth")
    write_yaml_line(out_stream, "⅖", "!N⠃N⠌N⠑","2156", "Vulgar Fraction Two Fifths")
    write_yaml_line(out_stream, "⅗", "!N⠉N⠌N⠑","2157", "Vulgar Fraction Three Fifths")
    write_yaml_line(out_stream, "⅘", "!N⠙N⠌N⠑","2158", "Vulgar Fraction Four Fifths")
    write_yaml_line(out_stream, "⅙", "!N⠁N⠌N⠋","2159", "Vulgar Fraction One Sixth")
    write_yaml_line(out_stream, "⅚", "!N⠑N⠌N⠋","215A", "Vulgar Fraction Five Sixths")
    write_yaml_line(out_stream, "⅛", "!N⠁N⠌N⠓","215B", "Vulgar Fraction One Eighth")
    write_yaml_line(out_stream, "⅜", "!N⠉N⠌N⠓","215C", "Vulgar Fraction Three Eighths")
    write_yaml_line(out_stream, "⅝", "!N⠑N⠌N⠓","215D", "Vulgar Fraction Five Eighths")
    write_yaml_line(out_stream, "⅞", "!N⠛N⠌N⠓","215E", "Vulgar Fraction Seven Eighths")

    write_yaml_line(out_stream, "↉", "!N⠚N⠌N⠑","2189", "Vulgar Fraction Zero Thirds")
    out_stream.write("\n")


def write_special_lines(out_stream):
    # chars that can be in a number needs a special test when in a number
    for ch in [ [" ", "N⠐", "W", "mn", "Space"],
                [" ", "N⠐", "W", "mn", "Non-breaking Space"],
                [",", "N⠂", ",", "mn", "Comma"],
                [".", "N⠲", ".", "mn", "Period"],
                ["-", "⠐⠤", "⠤", "mo", "Minus sign or hyphen"],
            ]:
        description = "" if ch[4]=="" else '(' + ch[4] + ')'
        out_stream.write(' - "{}":                        # 0x{} {}\n'.format(ch[0], hex(ord(ch[0]))[2:], description))
        out_stream.write('     - test:\n')
        out_stream.write('        if: "self::m:{}"\n'.format(ch[3]))
        out_stream.write('        then: [t: "{}"]\n'.format(ch[1]))
        out_stream.write('        else: [t: "{}"]\n'.format(ch[2]))



import re
INPUT_LINE = re.compile('[^"]*"(?P<char>.)"[^"]*"(?P<nemeth>[^"]*)"](?P<rest>.*)')

import yaml
def nemeth_shape_to_ueb_shape(in_file, out_file):
    with open(out_file, 'w', encoding="utf8") as out_stream:
        with open(in_file, encoding="utf-8-sig") as in_stream:
            for line in in_stream:
                out_stream.write(convert_line_to_ueb(line))

def convert_line_to_ueb(line: str):
    m = INPUT_LINE.match(line).groupdict()
    return ' - "{}": "{}"{}\n'.format(m["char"], covert_nemeth_to_ueb(m["nemeth"]), m["rest"])

SHAPE_RE = re.compile(
    ("("
        "⠫"
        "(?P<filled>[⠸⠨]?)"
        "(?P<shape>[⠁⠄⠙⠑⠊⠇⠕⠏⠛⠟⠗⠓⠎⠵⠞⠉⠪⠲])"
        "(((?P<imod>(⠸⠫))|(?P<smod>⠨))(?P<char>[^⠻]+))"
        "(?P<end>⠻?)"
      ")")
)

ARROW_RE = re.compile(
    ("("
        "⠫"
        "(?P<dir>[⠣⠩⠘⠰]?)"
        "(?P<bold>⠸?)"
        "(?P<lHalf>[⠠⠈]?)"
        "(?P<lKind>[⠿⠳⠪⠯]?[⠿⠳⠪⠯]?)"
        "(?P<shaft>(⠢⠔|⠒⠀⠒|⠂⠂⠂|⠶⠶⠶|⠶⠶|⠶|⠒⠒⠒|⠒⠒|⠒|⠔⠒⠢)?)"
        "(?P<rHalf>[⠠⠈]?)"
        "(?P<rKind>[⠿⠳⠕⠽]?[⠿⠳⠕⠽]?)"
     ")")
)

def covert_nemeth_to_ueb(nemeth: str):
    parts = SHAPE_RE.match(nemeth)
    if parts:
        return shape_start(parts["filled"]) + \
            shape_convert(parts["shape"]) + \
            interior_char(parts["imod"], parts["char"]) + \
            structural_char(parts["smod"], parts["char"]) + \
            "T"

    parts = ARROW_RE.match(nemeth)
    if parts:
        return arrow_start(parts["bold"]) + \
            arrow_convert(parts["arrow"]) + \
            interior_char(parts["imod"], parts["char"]) + \
            structural_char(parts["smod"], parts["char"]) + \
            "T"
    
    # error
    return "NONE!!!"

SHAPE_START = {
    "⠸": "⠸⠫", # filled
    "⠨": "⠨⠫", # shaded
}
def shape_start(nemeth_filled):
    return "S" + SHAPE_START.get(nemeth_filled, "1⠫")
    
SHAPE = {
    "⠞": "⠼⠉",   # (equilateral) triangle  
    "⠲": "⠼⠙",   # square
    "⠢": "⠼⠑ ",   # pentagon
    "⠖": "⠼⠋",   # hexagon
    "⠦": "⠼⠓",   # octagon
    "⠉": "⠿",     # circle
    "⠛": "⠈⠼⠙",   # parallelogram
}
def shape_convert(nemeth_shape):
    return SHAPE.get(nemeth_shape, "UNKNOWN SHAPE!")

def interior_char(indicator, ch):
    if indicator == None:
        return ""
    else:
        return "1⠪" + translate(ch)

def structural_char(indicator, ch):
    if indicator == None:
        return ""
    else:
        return "structural_char" + translate(ch)


def arrow_start(bold):
    if bold == None:
        return "1⠳"
    else:
        return "1⠘⠳"
   




def translate(ch):
    return "**" + ch + "**"   # FIX: IMPLEMENT (needs to convert Nemeth to UEB)

write_ueb_yaml("ueb.csv", "unicode.yaml")
# nemeth_shape_to_ueb_shape('shapes.yaml', 'ueb-shapes.yaml')

