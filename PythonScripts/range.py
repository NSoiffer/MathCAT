# Generate chars in a range. Useful for pasting in for 'translate' strings in unicode-full.yaml (etc)

# Accepts either chars or hex (0xdddd)
def char_range(first_char, last_char):
    first_char = first_char if isinstance(first_char, int) else ord(first_char)
    last_char = last_char if isinstance(last_char, int) else ord(last_char)
    for i in range(first_char, last_char+1):
        print(chr(i), end='')
    print('')

LOWER_CASE_WITH_TEXT = """
 - "{}-{}":    # {} - {}
    - t: "{}"
    - spell: "translate('.', '{}', '{}')"
"""
LOWER_CASE = """
 - "{}-{}":    # {} - {}
    - spell: "translate('.', '{}', '{}')"
"""
UPPER_CASE_WITH_TEXT = """
 - "{}-{}":    # {} - {}
    - t: "{}"
    - test: 
          if: "$SpeechOverrides_CapitalLetters = ''"
          then_test:
            if: "$Impairment = 'Blindness'"
            then: [t: "cap"]
          else: [x: "$SpeechOverrides_CapitalLetters"] 
    - pitch:
        value: "$CapitalLetters_Pitch"
        replace: [spell: "translate('.', '{}', '{}')"]
"""
UPPER_CASE = """
 - "{}-{}":    # {} - {}
    - test: 
          if: "$SpeechOverrides_CapitalLetters = ''"
          then_test:
            if: "$Impairment = 'Blindness'"
            then: [t: "cap"]
          else: [x: "$SpeechOverrides_CapitalLetters"] 
    - pitch:
        value: "$CapitalLetters_Pitch"
        replace: [spell: "translate('.', '{}', '{}')"]
"""

# Accepts either chars or hex (0xdddd) for 'first_char'
# 'n_chars' can be number of chars or last char (either number or char)
# 'case' should be 'upper' or 'lower'
def generate(first_char, n_chars, text="", case="upper", first_trans='a'):
    first_char = first_char if isinstance(first_char, int) else ord(first_char)
    first_trans = first_char if isinstance(first_trans, int) else ord(first_trans)
    n_chars = n_chars if isinstance(n_chars, int) else ord(n_chars)
    if n_chars > 100:
        n_chars = n_chars+1 - first_char  # convert to offset

    chars = ''.join(list(map(chr, range(first_char, first_char+n_chars))))
    translated_chars = ''.join(list(map(chr, range(first_trans, first_trans+n_chars))))
    template = UPPER_CASE_WITH_TEXT
    if case=='lower':
        template = LOWER_CASE_WITH_TEXT if text else LOWER_CASE
    else:
        template = UPPER_CASE_WITH_TEXT if text else UPPER_CASE
    if text:
        print(template.format(chars[0], chars[-1], hex(first_char), hex(first_char+n_chars-1),
            text, chars, translated_chars))
    else:
        print(template.format(chars[0], chars[-1], hex(first_char), hex(first_char+n_chars-1),
            chars, translated_chars))

    print('  let expr = "<math> <mi>{}</mi><mo>,</mo><mi>{}</mi></math>";'
        .format(chars[0], chars[-1]))
    print() # get extra newline


char_range('Α', 'Ω')
# generate('𝞐',25, text='bold italic', case='upper', first_trans='α')
# generate('',25, text='bold italic', case='upper', first_trans='α')
# generate('𝞪',25, text='bold italic', case='lower', first_trans='α')
# generate('',25, text='bold italic', case='lower', first_trans='α')
