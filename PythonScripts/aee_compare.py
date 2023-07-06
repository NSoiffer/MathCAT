import yaml
from AEEChars import vchar_data

COMPUTER_BRAILLE_TO_UNICODE = " a1b'k2l`cif/msp\"e3h9o6r~djg>ntq,*5<-u8v.%{$+x!&;:4|0z7(_?w}#y)="
def computer_braille_to_unicode(ascii: str):
    result = "";
    ascii = ascii.lower()
    for ch in ascii:
        try:
            result += chr(COMPUTER_BRAILLE_TO_UNICODE.index(ch) + 0x2800)
        except:
            print("problem translating '{}'".format(ch))
            exit()
    return result

def get_AEE_UEB_data() -> dict[str, str]:
    aee_dict = {}
    for entry in vchar_data:
        aee_dict[entry[0]] = computer_braille_to_unicode(entry[1])
    return aee_dict

UEB_EXTRA_CHAR_DICT = {
    "S": "SSS",    # sans-serif
    "B": "⠘",     # bold
    "𝔹": "⠈BBB",     # blackboard
    "T": "⠈",     # script
    "I": "⠨",     # italic
    "R": "",      # roman
    # "E": "⠰",     # English
    "1": "⠰",     # Grade 1 symbol
    "L": "",     # Letter left in to assist in locating letters
    "D": "DDD",     # German (Deutsche)
    "G": "⠨",     # Greek
    # "V": "⠨⠈",    # Greek Variants
    # "H": "⠠⠠",    # Hebrew
    # "U": "⠈⠈",    # Russian
    "C": "⠠",      # capital
    "𝐶": "⠠",      # capital that never should get word indicator (from chemical element)
    "N": "⠼",     # number indicator
    "t": "⠱",     # shape terminator
    "W": "⠀",     # whitespace
    "𝐖": "⠀",     # whitespace
    "s": "⠆",     # typeface single char indicator
    "w": "⠂",     # typeface word indicator
    "e": "⠄",     # typeface & capital terminator 
    "o": "",       # flag that what follows is an open indicator (used for standing alone rule)
    "c": "",       # flag that what follows is an close indicator (used for standing alone rule)
    "b": "",       # flag that what follows is an open or close indicator (used for standing alone rule)
    ",": "⠂",     # comma
    ".": "⠲",     # period
    "-": "-",     # hyphen
    "—": "⠠⠤",   # normal dash (2014) -- assume all normal dashes are unified here [RUEB appendix 3]
    "―": "⠐⠠⠤",  # long dash (2015) -- assume all long dashes are unified here [RUEB appendix 3]
    "#": "",      # signals end of script
}

import re
# not quite right match for second digit in denom, but good enough for here
VULGAR_FRACTION = re.compile('#N([⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚])N⠌N([⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚])N*([⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚])*')

def remove_mathcat_indicators(braille_entry: str) -> str:
    # remove 'N' hack for vulgar fractions (#Nb+N⠌Nb+)
    braille_entry = VULGAR_FRACTION.sub("N\\1⠌\\2\\3", braille_entry)
    for letter, braille in UEB_EXTRA_CHAR_DICT.items():
        braille_entry = braille_entry.replace(letter, braille)
    return braille_entry

import os
def get_mathcat_data_from_file(file, braille_code: str) -> dict[str,str]:
    with open(file, 'r', encoding="utf-8") as file:
        names = {}
        definitions = yaml.safe_load(file)
        for definition in definitions:
            # definition looks like {'0', [{'t': 'N⠴'}]}
            if len(definition) != 1:
                print("*** problem with definition: " + definition)
            char = list(definition.keys())[0]
            replacement_dict = list(definition.values())[0][0]
            braille_entry = replacement_dict.get('t')
            if braille_entry:
                # need to replace "C", etc, with the expansion
                braille_entry = remove_mathcat_indicators(braille_entry)
            else:
                # special case for chemistry elements
                try:
                    braille_entry = replacement_dict['test']['else'][0]['t']
                    braille_entry = remove_mathcat_indicators(braille_entry)
                except:
                    # not simple text entry
                    print("*** problem with braille entry KEY of definition: {}".format(definition))
            names[char] = braille_entry
        return names

def get_mathcat_data(dir: str, braille_code: str) -> dict[str,str]:        
    dict = get_mathcat_data_from_file(os.path.join(dir, "unicode.yaml"), braille_code)
    # dict.update(get_mathcat_data_from_file(os.path.join(dir, "unicode-full.yaml"), braille_code))
    return dict

def write_dictionary(stream, dict: dict[str,str]):
    stream.write("{\n")
    for key,value in dict.items():
        stream.write("  {{{}: {}}},\n".format(key,value))
    stream.write("}\n")

def compare_UEB_defs():
    aee_dict = get_AEE_UEB_data()
    mc_dict = get_mathcat_data("../Rules/Braille/UEB", "UEB")

    only_in_aee = {}
    only_in_mc = {}
    differs = {}
    for aee_char, aee_braille in aee_dict.items():
        mc_braille = mc_dict.get(aee_char)
        if mc_braille:
            if aee_braille != mc_braille:
                differs[aee_char] = {"aee": aee_braille, "MathCAT": mc_braille}
        else:
            only_in_aee[aee_char] = aee_braille
    for mc_char, mc_braille in mc_dict.items():
        if not(mc_dict.get(mc_char)):
            only_in_mc[mc_char] = mc_braille
    
    with open("aee-mathcat-compare.txt", 'w', encoding='utf8') as out_stream:
        out_stream.write("\nDIFFERENCES\n")
        write_dictionary(out_stream, differs)
        out_stream.write("\nOnly in AEE\n")
        write_dictionary(out_stream, only_in_aee)
        out_stream.write("\nOnly in MathCAT\n")
        write_dictionary(out_stream, only_in_mc)

def create_aee_rust_tests(in_file: str, out_file: str):
    import xml.etree.ElementTree as ET
    tree = ET.parse(in_file)
    root = tree.getroot()
    count = 0
    with open(out_file, 'w', encoding='utf8') as out_stream:
        out_stream.write("use crate::common::*;\n")
        for test in root:
            out_stream.write("\n#[test]\n")
            out_stream.write("fn aee_{:04d}() {{\n".format(count))
            mathml = ET.tostring(test[1], encoding='unicode').replace('"', '\'').replace('\\', '\\\\').rstrip()
            out_stream.write("    let expr = \"{}\";\n".format(mathml))
            nemeth = test[2].text.replace('\\', '\\\\')
            out_stream.write("    test_braille(\"Nemeth\", expr, \"{}\");\n".format(test[2].text))
            out_stream.write("}\n")
            count += 1


# compare_UEB_defs()
create_aee_rust_tests("cptob.xml", "AEE.rs")