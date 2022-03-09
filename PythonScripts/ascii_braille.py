UNICODE_TO_ASCII = """ a1b\'k2l@cif/msp"e3h9o6r^djg>ntq,*5<-u8v.%[$+x!&;:4\\0z7(_?w]#y)="""
def unicode_to_ascii(unicode: str):
    result = "";
    for ch in unicode:
        if ord(ch) < 128:
            result += '_'
        else:
            i = ord(ch) - 0x2800
            result += UNICODE_TO_ASCII[i]
    print( result )

# convert a bunch of text that is in Unicode braille to ASCII braille
# typically uses as text_u2a(""" lines of text """)
def text_u2a(unicode: str):
    result = "";
    for ch in unicode:
        if ord(ch) < 128:
            result += ch
        else:
            i = ord(ch) - 0x2800
            result += UNICODE_TO_ASCII[i]
    print( result )

def u2a(unicode:str):
    return unicode_to_ascii(unicode)

ASCII_TO_UNICODE = "⠀⠮⠐⠼⠫⠩⠯⠄⠷⠾⠡⠬⠠⠤⠨⠌⠴⠂⠆⠒⠲⠢⠖⠶⠦⠔⠱⠰⠣⠿⠜⠹⠈⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚⠅⠇⠍⠝⠕⠏⠟⠗⠎⠞⠥⠧⠺⠭⠽⠵⠪⠳⠻⠘⠸"
def ascii_to_unicode(ascii: str):
    result = "";
    ascii = ascii.upper()
    for ch in ascii:
        i = ord(ch) - 32
        result += ASCII_TO_UNICODE[i]
    return result

def a2u(ascii:str):
    return ascii_to_unicode(ascii)

def generate_ascii_to_unicode():
    i = 0
    to_unicode = ['X' for i in range(32, 96)]
    for ch in UNICODE_TO_ASCII:
        to_unicode[ord(ch)-32] = chr(0x2800 + i)
        i += 1

    result = ""
    for ch in to_unicode:
        result += ch
    print(result)

