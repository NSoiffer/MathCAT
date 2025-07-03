import json
import re


UNICODE_TO_ASCII = """ a1b\'k2l@cif/msp"e3h9o6r^djg>ntq,*5<-u8v.%[$+x!&;:4\\0z7(_?w]#y)="""


def unicode_to_ascii(unicode: str):
    result = ""
    for ch in unicode:
        if ord(ch) < 128:
            result += '_'
        else:
            i = ord(ch) - 0x2800
            result += UNICODE_TO_ASCII[i]
    print(result)


def text_u2a(unicode: str):
    """convert a string text that is in Unicode braille to ASCII braille.
       the result is printed"""
    result = ""
    for ch in unicode:
        if ord(ch) < 128:
            result += ch
        else:
            i = ord(ch) - 0x2800
            result += UNICODE_TO_ASCII[i]
    print(result)


def u2a(unicode: str):
    return unicode_to_ascii(unicode)


ASCII_TO_UNICODE = "⠀⠮⠐⠼⠫⠩⠯⠄⠷⠾⠡⠬⠠⠤⠨⠌⠴⠂⠆⠒⠲⠢⠖⠶⠦⠔⠱⠰⠣⠿⠜⠹⠈⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚⠅⠇⠍⠝⠕⠏⠟⠗⠎⠞⠥⠧⠺⠭⠽⠵⠪⠳⠻⠘⠸"


def ascii_to_unicode(ascii: str):
    result = ""
    ascii = ascii.upper()
    for ch in ascii:
        i = ord(ch) - 32
        try:
            result += ASCII_TO_UNICODE[i]
        except IndexError:
            # hack for https://www.brailleauthority.org/sites/default/files/chemistry/Chemical%20Notation%20Usin   # noqa: E501
            if ch == '¡':
                result += '⠀'
            else:
                print(f"did not find '{ch}' in ASCII_TO_UNICODE table")
    return result


def a2u(ascii: str):
    return ascii_to_unicode(ascii)


def a2u_loop():
    text = input("text: ")
    while text != "":
        print(ascii_to_unicode(text))
        text = input("text: ")


def read_euro_braille_file() -> dict[str, str]:
    # I had to purge the control chars from the file because I was getting an error like:
    #   json.decoder.JSONDecodeError: Invalid \escape: line 12 column 4 (char 125)
    with open("euro-braille-dict.txt", 'r', encoding='utf8') as in_stream:
        return json.loads(in_stream.read(), strict=True)


ASCII_TO_EURO_BRAILLE: dict[str, str] = read_euro_braille_file()


def ascii_to_euro_braille(ascii: str):
    result = ""
    for ch in ascii:
        result += str(ASCII_TO_EURO_BRAILLE.get(ch))
    return result


def a2eb(ascii: str):
    return ascii_to_euro_braille(ascii)


def a2eb_loop():
    text = input("text: ")
    while text != "":
        print(ascii_to_euro_braille(text))
        text = input("text: ")


def swap_dict(ascii_to_braille: dict[str, str]) -> dict[str, str]:
    braille_to_ascii = {}
    for (key, value) in ascii_to_braille.items():
        if 32 <= ord(key) and ord(key) < 128:
            braille_to_ascii[value] = key
    return braille_to_ascii


EURO_BRAILLE_TO_ASCII: dict[str, str] = swap_dict(ASCII_TO_EURO_BRAILLE)


def euro_braille_to_ascii(braille: str):
    result = ""
    for ch in braille:
        try:
            result += str(EURO_BRAILLE_TO_ASCII.get(ch))
        except IndexError:
            print(f"couldn't find mapping for '{ch}/{hex(ord(ch))}")
    return result


def eu2a(braille: str):
    return euro_braille_to_ascii(braille)


def eu2a_loop():
    text = input("text: ")
    while text != "":
        print(euro_braille_to_ascii(text))
        text = input("text: ")


# Major hack
# ONCE PDF uses a mapping that seems to be based on the Spanish char encoding. This is a bit like ASCII braille, but different
# Here we start with the ASCII encoding, stretch it to 128, then remap characters
#   set_char() is used to override a spot
#   SPANISH_REMAP is used to take non-ASCII chars back down to the ASCII braille mapping if it isn't overridden already.
# It's all a hack. It would be best if I could find some string encoding like for ASCII braille,
#   but with large gaps due to accented chars
# The remap list has grown large, so maybe just have it map to Unicode and be done with the array lookup.
SPANISH_TO_UNICODE = ''
# def initialize_spanish():
#     def set_char(old:str, new:str):
#         global SPANISH_TO_UNICODE
#         index = ord(old) - 32
#         SPANISH_TO_UNICODE = SPANISH_TO_UNICODE[:index] + new + SPANISH_TO_UNICODE[index+1:]

#     global SPANISH_TO_UNICODE
#     SPANISH_TO_UNICODE = ASCII_TO_UNICODE + ''.join(list(map(lambda i: chr(i), range(32+64,128))))
#     set_char('@', '⠐')
#     set_char('?', '⠢')
#     set_char('!', '⠖')
#     set_char('+', '⠖')   # same as '!'
#     set_char(',', '⠂')
#     set_char('.', '⠄')
#     set_char(':', '⠒')
#     set_char('}', '⠔')
#     set_char('"', '⠦')
#     set_char('{', '⠨')
#     set_char('a', '⠸')
#     set_char('e', '⠮')
#     set_char('f', '⠱')
#     set_char('_', '⠠')
#     set_char("`", '⠈')
#     set_char("=", '⠶')


# initialize_spanish()
# SPANISH_REMAP = {'¿': '?', 'Á': '(', 'Â': '*', 'É': 'e', 'Ë': '$', 'Ü': '\\', 'Ú': ')', 'Û': 'f',
#                  'Ñ': ']', 'Í': '/', 'Ó': '+', 'Ç': '&', '÷': '4', '°': '0',
#                  '(': '<', ')': '>', '¬': ' ', '%': 'a', ';': '2', '¨': ';', '*': '8', 'Ö': '[',
#                 }
# def spanish_to_unicode(ascii: str):
#     result = "";
#     ascii = ascii.upper()
#     for ch in ascii:
#         found = SPANISH_REMAP.get(ch)
#         if found:
#             ch = found
#         i = ord(ch) - 32
#         result += SPANISH_TO_UNICODE[i]
#     return result

REMAP = {
    ',': '1',  ';': '2', ':': '3', '*': '4',  '?': '5', '¿': '5', '+': '6', '=': '7', '[': '8', '}': '9',  ']': '0',
    '(': '<',  ')': '>',
    '.': "'", '_': ',', '@': '"', '{': '.', '%': '_', '\\': '=',
    '¾': '@',
    '×': '*',
    '1': '*',
    '°': 'A', 'º': ';', '§': '*',
    'Í': '/', 'Ü': '\\', 'Ý': ':', 'Ë': '$', 'Ó': '+', 'Ö': '[', 'Ç': '&', 'Á': '(', 'Ã': '*', 'Ú': ')', 'Ñ': ']',
    '»': ';',
}


def remap(ch: str) -> str:
    global REMAP
    ch = ch.upper()
    for k, v in REMAP.items():
        if k == ch:
            return ch.replace(k, v)
    return ch


def spanish_to_unicode(ascii: str):
    result = ""
    for ch in ascii:
        i = ord(remap(ch)) - 32
        try:
            result += ASCII_TO_UNICODE[i]
        except IndexError:
            print(f"ch='{ch}' is not ASCII braille char")
    return result


def s2u(ascii: str):
    return spanish_to_unicode(ascii)


def s2u_loop():
    user_input = "foo"
    while user_input != "":
        user_input = input('s2u: ')
        print(spanish_to_unicode(user_input))


def dots_to_unicode(dots: int):
    answer = 0
    while dots > 0:
        digit = dots % 10
        dots = (dots-digit)//10
        answer += pow(2, digit-1)
    return chr(0x2800+answer)


def d2u(dots: int):
    return dots_to_unicode(dots)


def dl2u(dots: list):
    return " ".join(list(map(dots_to_unicode, dots)))


# takes a string like 4-346-15
def ds2u(dots: str):
    return "".join(list(map(lambda s: dots_to_unicode(int(s)), re.split(r'\D+', dots))))
