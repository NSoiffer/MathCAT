# convert CSV file for Unicode -> UEB mappings
#  this file doesn't include digits and letters, so they are added separately
# import csv
from ascii_braille import ascii_to_unicode

def write_ueb_yaml(in_file, out_file):
    with open(out_file, 'w', encoding="utf8") as out_stream:
        with open(in_file, encoding="utf-8") as yaml_in_file:
            out_stream.write("---\n")
            for line in yaml_in_file:
                out_stream.write(line)
        write_letters_and_digits(out_stream)



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
    write_letter_chars(out_stream, my_ascii_to_unicode(small_latin), 'A', "CL")

    # no grade 1 indicator for Greek letters -- capitalization comes first
    write_letter_chars(out_stream, my_ascii_to_unicode(small_greek), 'α', "GL")
    write_letter_chars(out_stream, my_ascii_to_unicode(small_greek), 'Α', "CGL")   # capital before Greek
    write_special_greeks(out_stream)

    # various typeforms in the math alphanumerics
    # bold
    write_letter_chars(out_stream, my_ascii_to_unicode(small_latin), '𝐚', "BL")
    write_letter_chars(out_stream, my_ascii_to_unicode(small_latin), '𝐀', "BCL")
    # italic
    write_letter_chars(out_stream, my_ascii_to_unicode(small_latin), '𝑎', "IL")
    write_letter_chars(out_stream, my_ascii_to_unicode(small_latin), '𝐴', "ICL")
    # bold italic
    write_letter_chars(out_stream, my_ascii_to_unicode(small_latin), '𝒂', "BIL")
    write_letter_chars(out_stream, my_ascii_to_unicode(small_latin), '𝑨', "BICL")
    # script
    write_letter_chars(out_stream, my_ascii_to_unicode(small_latin), '𝒶', "TsL")
    write_letter_chars(out_stream, my_ascii_to_unicode(small_latin), '𝒜', "TsCL")
    # a few script are out of sequence
    write_yaml_line(out_stream, "ℊ", "TL⠛⠁","210a", "Script Small G")
    write_yaml_line(out_stream, "ℋ", "TCL⠓⠁","210b", "Script Capital H")
    write_yaml_line(out_stream, "ℒ", "TCL⠇","2113", "Script Capital L")
    write_yaml_line(out_stream, "ℓ", "TL⠇","2113", "Script Small L")
    write_yaml_line(out_stream, "℘", "TCL⠏","2118", "Script Capital P")
    write_yaml_line(out_stream, "ℛ", "TCL⠗","211B", "Script Capital R")
    write_yaml_line(out_stream, "ℯ", "TL⠑","212F", "Script Small E")
    write_yaml_line(out_stream, "ℰ", "TCL⠑","2130", "Script Capital E")
    write_yaml_line(out_stream, "ℱ", "TCL⠋","2131", "Script Capital F")
    write_yaml_line(out_stream, "ℳ", "TCL⠍","2133", "Script Capital M")
    write_yaml_line(out_stream, "ℴ", "TL⠕","2134", "Script Small O")


    # bold script
    write_letter_chars(out_stream, my_ascii_to_unicode(small_latin), '𝓪', "BTsL")
    write_letter_chars(out_stream, my_ascii_to_unicode(small_latin), '𝓐', "BTsCL")
    # fraktur
    write_letter_chars(out_stream, my_ascii_to_unicode(small_latin), '𝔞', "DL")
    write_letter_chars(out_stream, my_ascii_to_unicode(small_latin), '𝔄', "DCL")
    # a few fraktur are out of sequence
    write_yaml_line(out_stream, "ℌ", "DCL⠓","210C", "Fraktur Capital H")
    write_yaml_line(out_stream, "ℑ", "DCL⠊","2111", "Fraktur Capital I")
    write_yaml_line(out_stream, "ℜ", "DCL⠗","211C", "Fraktur Capital R")
    write_yaml_line(out_stream, "ℨ", "DCL⠵","2128", "Fraktur Capital Z")
    write_yaml_line(out_stream, "ℭ", "DCL⠉","22DC", "Fraktur Capital C")
   

    #double struck caps have some chars out of sequence, so these have to be done separately
    write_letter_chars(out_stream, my_ascii_to_unicode(small_latin), '𝕒', "𝔹L")
    write_yaml_line(out_stream, "𝔸", "𝔹CL⠁","1d538", "")
    write_yaml_line(out_stream, "𝔹", "𝔹CL⠃","1d539", "")
    write_yaml_line(out_stream, "ℂ", "𝔹CL⠉","2102", "")
    write_yaml_line(out_stream, "𝔻", "𝔹CL⠙","1d53b", "")
    write_yaml_line(out_stream, "𝔼", "𝔹CL⠑","1d53c", "")
    write_yaml_line(out_stream, "𝔽", "𝔹CL⠋","1d53d", "")
    write_yaml_line(out_stream, "𝔾", "𝔹CL⠛","1d53e", "")
    write_yaml_line(out_stream, "ℍ", "𝔹CL⠓","210d", "")
    write_yaml_line(out_stream, "𝕀", "𝔹CL⠊","1d540", "")
    write_yaml_line(out_stream, "𝕁", "𝔹CL⠚","1d541", "")
    write_yaml_line(out_stream, "𝕂", "𝔹CL⠅","1d542", "")
    write_yaml_line(out_stream, "𝕃", "𝔹CL⠇","1d543", "")
    write_yaml_line(out_stream, "𝕄", "𝔹CL⠍","1d544", "")
    write_yaml_line(out_stream, "ℕ", "𝔹CL⠝","2115", "")
    write_yaml_line(out_stream, "𝕆", "𝔹CL⠕","1d546", "")
    write_yaml_line(out_stream, "ℙ", "𝔹CL⠏","2119", "")
    write_yaml_line(out_stream, "ℚ", "𝔹CL⠟","211a", "")
    write_yaml_line(out_stream, "ℝ", "𝔹CL⠗","211d", "")
    write_yaml_line(out_stream, "𝕊", "𝔹CL⠎","1d54a", "")
    write_yaml_line(out_stream, "𝕋", "𝔹CL⠞","1d54b", "")
    write_yaml_line(out_stream, "𝕌", "𝔹CL⠥","1d54c", "")
    write_yaml_line(out_stream, "𝕍", "𝔹CL⠧","1d54d", "")
    write_yaml_line(out_stream, "𝕎", "𝔹CL⠺","1d54e", "")
    write_yaml_line(out_stream, "𝕏", "𝔹CL⠭","1d54f", "")
    write_yaml_line(out_stream, "𝕐", "𝔹CL⠽","1d550", "")
    write_yaml_line(out_stream, "ℤ", "𝔹CL⠵","2124", "")

    # bold fraktur
    write_letter_chars(out_stream, my_ascii_to_unicode(small_latin), '𝖆', "BDL")
    write_letter_chars(out_stream, my_ascii_to_unicode(small_latin), '𝕬', "BDCL")
    # sans-serif
    write_letter_chars(out_stream, my_ascii_to_unicode(small_latin), '𝖺', "SL")
    write_letter_chars(out_stream, my_ascii_to_unicode(small_latin), '𝖠', "SCL")
    # bold sans-serif
    write_letter_chars(out_stream, my_ascii_to_unicode(small_latin), '𝗮', "BSL")
    write_letter_chars(out_stream, my_ascii_to_unicode(small_latin), '𝗔', "BSCL")
    # italic sans-serif
    write_letter_chars(out_stream, my_ascii_to_unicode(small_latin), '𝘢', "ISL")
    write_letter_chars(out_stream, my_ascii_to_unicode(small_latin), '𝘈', "ISCL")
    # bold italic sans-serif
    write_letter_chars(out_stream, my_ascii_to_unicode(small_latin), '𝙖', "BILS")
    write_letter_chars(out_stream, my_ascii_to_unicode(small_latin), '𝘼', "BILSC")
    # monospaced -- ignore and treat as standard letter
    write_letter_chars(out_stream, my_ascii_to_unicode(small_latin), '𝚊', "L")
    write_letter_chars(out_stream, my_ascii_to_unicode(small_latin), '𝙰', "CL")

    # map dotless i, j to just the letters i, j
    write_yaml_line(out_stream, "𝚤", "L⠊", "1d6a4", "dotless i")
    write_yaml_line(out_stream, "𝚥", "L⠚", "1d6a5", "dotless j")

    write_letter_chars(out_stream, my_ascii_to_unicode(small_greek), '𝛂', "BGL")
    write_letter_chars(out_stream, my_ascii_to_unicode(small_greek), '𝚨', "BCGL")

    write_letter_chars(out_stream, my_ascii_to_unicode(small_greek), '𝛼', "IGL")
    write_letter_chars(out_stream, my_ascii_to_unicode(small_greek), '𝛢', "ICGL")

    write_letter_chars(out_stream, my_ascii_to_unicode(small_greek), '𝜶', "BIGL")
    write_letter_chars(out_stream, my_ascii_to_unicode(small_greek), '𝜜', "BICGL")

    write_letter_chars(out_stream, my_ascii_to_unicode(small_greek), '𝝰', "BSGL")
    write_letter_chars(out_stream, my_ascii_to_unicode(small_greek), '𝝖', "BSCGL")

    write_letter_chars(out_stream, my_ascii_to_unicode(small_greek), '𝞪', "BISGL")
    write_letter_chars(out_stream, my_ascii_to_unicode(small_greek), '𝞐', "BISCGL")


    write_letter_chars(out_stream, digits, '𝟎', "BN")
    write_letter_chars(out_stream, digits, '𝟘', "𝔹N")
    write_letter_chars(out_stream, digits, '𝟢', "SN")
    write_letter_chars(out_stream, digits, '𝟬', "BSN")
    write_letter_chars(out_stream, digits, '𝟶', "N")


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
    write_yaml_line(out_stream, "µ", "GL" + ascii_to_unicode("m"),"00B5", "Micro (Greek mu)")
    write_yaml_line(out_stream, "Ω", "CGL" + ascii_to_unicode("m"),"2126", "Ohm sign (capital Greek omega)")
    write_yaml_line(out_stream, "∆", "CGL" + ascii_to_unicode("d"),"2206", "Increment (capital Greek delta)")
    write_yaml_line(out_stream, "∏", "CGL" + ascii_to_unicode("p"),"220F", "Product (capital Greek pi)")
    write_yaml_line(out_stream, "∑", "CGL" + ascii_to_unicode("s"),"2211", "Sum (capital Greek sigma)")
    out_stream.write("\n")

def  write_vulgar_fractions(out_stream):
    write_yaml_line(out_stream, "¼", "#N⠁N⠌N⠙","00BC", "Vulgar Fraction One Quarter")
    write_yaml_line(out_stream, "½", "#N⠁N⠌N⠃","00BD", "Vulgar Fraction One Half")
    write_yaml_line(out_stream, "¾", "#N⠉N⠌N⠙","00BE", "Vulgar Fraction Three Quarters")

    write_yaml_line(out_stream, "⅐", "#N⠁N⠌N⠛","2150", "Vulgar Fraction One Seventh")
    write_yaml_line(out_stream, "⅑", "#N⠁N⠌N⠊","2151", "Vulgar Fraction One Ninth")
    write_yaml_line(out_stream, "⅒", "#N⠁N⠌N⠁N⠚","2152", "Vulgar Fraction One Tenth")
    write_yaml_line(out_stream, "⅓", "#N⠁N⠌N⠉","2153", "Vulgar Fraction One Third")
    write_yaml_line(out_stream, "⅔", "#N⠃N⠌N⠉","2154", "Vulgar Fraction Two Thirds")
    write_yaml_line(out_stream, "⅕", "#N⠁N⠌N⠑","2155", "Vulgar Fraction One Fifth")
    write_yaml_line(out_stream, "⅖", "#N⠃N⠌N⠑","2156", "Vulgar Fraction Two Fifths")
    write_yaml_line(out_stream, "⅗", "#N⠉N⠌N⠑","2157", "Vulgar Fraction Three Fifths")
    write_yaml_line(out_stream, "⅘", "#N⠙N⠌N⠑","2158", "Vulgar Fraction Four Fifths")
    write_yaml_line(out_stream, "⅙", "#N⠁N⠌N⠋","2159", "Vulgar Fraction One Sixth")
    write_yaml_line(out_stream, "⅚", "#N⠑N⠌N⠋","215A", "Vulgar Fraction Five Sixths")
    write_yaml_line(out_stream, "⅛", "#N⠁N⠌N⠓","215B", "Vulgar Fraction One Eighth")
    write_yaml_line(out_stream, "⅜", "#N⠉N⠌N⠓","215C", "Vulgar Fraction Three Eighths")
    write_yaml_line(out_stream, "⅝", "#N⠑N⠌N⠓","215D", "Vulgar Fraction Five Eighths")
    write_yaml_line(out_stream, "⅞", "#N⠛N⠌N⠓","215E", "Vulgar Fraction Seven Eighths")

    write_yaml_line(out_stream, "↉", "#N⠚N⠌N⠑","2189", "Vulgar Fraction Zero Thirds")
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


# a function to take the shortform list from RUEB 10.9 and convert them into the form needed
# the form is 'L' followed by the braille version of the letter
shortforms = [
    'ab', 'abv', 'ac', 'acr', 'af', 'afn', 'afw', 'ag', 'ag/', 'al', 'alm', 'alr', 'alt', 'al?', 'alw',
    'bl', 'brl', 'cd', 'dcl', 'dclg', 'dcv', 'dcvg',
    'ei', 'fr', 'f/', 'gd', 'grt', 'hm', 'hmf', 'h]f', 'imm',
    'll', 'lr', 'myf', 'm*', 'm/', 'nec', 'nei', 'pd', 'p]cv', 'p]cvg', 'p]h',
    'qk', 'rcv', 'rcvg', 'rjc', 'rjcg', 'sd', 's*', 'td', 'tgr', 'tm', 'tn',
    'xf', 'xs', 'yr', 'yrf', 'yrvs', '!mvs', '*n', '%d', '?yf', '\\rvs', 'wd',
    '2c', '2f', '2h', '2l', '2n', '2s', '2t', '2y',
    '3cv', '3cvg', '"of' 
    ]
def str_to_unicode(str):
    return "".join(map(ascii_to_unicode, str))

def add_letter(str: str):
    return "".join(map(lambda ch: 'L'+ch, str))

def convert_shortforms():
    braille = list(map(str_to_unicode, shortforms))
    with_letters = map(add_letter, braille)
    print(list(with_letters))

# print(len(shortforms))
# convert_shortforms()

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

write_ueb_yaml("ueb-from-csv.yaml", "unicode.yaml")
# nemeth_shape_to_ueb_shape('shapes.yaml', 'ueb-shapes.yaml')

