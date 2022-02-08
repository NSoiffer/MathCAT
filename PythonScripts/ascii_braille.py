UNICODE_TO_ASCII = " A1B'K2L@CIF/MSP\"E3H9O6R^DJG>NTQ,*5<-U8V.%[$+X!&;:4\\0Z7(_?W]#Y)="
def unicode_to_ascii(unicode: str):
    result = "";
    for ch in unicode:
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

