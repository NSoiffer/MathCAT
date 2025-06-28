# convert CSV file for Unicode 
#  this file doesn't include digits and letters, so they are added separately
import csv
import html         # entity name conversion [html.unescape(str)]
import re
import sys

from attr import has
sys.stdout.reconfigure(encoding='utf-8')


def write_unicode_file(in_file, out_file, braille_code: str):
    """
    Read the csv file that comes from the braille math code repository (excel spreadsheet)
    write out lines like
    - "⇢": [t: "⠂⠂⠕"]                         # 0x21e2
    """
    with open(out_file, 'w', encoding="utf8") as out_stream:
        with open(in_file, encoding="utf8") as csv_file:
            csv_reader = csv.DictReader(csv_file, delimiter=',')
            line_count = 0
            output_line_strings = [] # want to sort them, so we store them
            for row in csv_reader:
                if line_count == 0:
                    print(f'Column names are {", ".join(row)}')
                elif row[braille_code]:
                    char_list = find_unicode_char(line_count, row, braille_code)
                    if len(char_list) == 1:
                        ch = char_list[0]
                        output_line_strings.append(f' - "{ch}": [t: "{row[braille_code]}"] {"# " + hex(ord(ch)):>32}\n')
                    elif len(char_list) > 1:
                        output_line_strings.append(f'#- "{char_list}": "{row[braille_code]}"\n')
                    else:  # didn't find anything to help understand what unicode character it should be -- output description
                        output_line_strings.append(f'#- "{row['Symbol Name']}": "{row[braille_code]}"\n')
                        
                line_count += 1
            print(f'Processed {line_count} lines.')

            output_line_strings.sort()
            [out_stream.write(line) for line in output_line_strings]
            # out_stream.write("---\n")
            # write_letters_and_digits(out_stream)
            # # entries are a list of numeric code point, char, full name, Nemeth, UEB
            # for entry in csv_reader:
            #     write_yaml_line(out_stream, unicode_char(entry), nemeth(entry), code_point(entry), unicode_name(entry))


def find_unicode_char(line_number: int, row: dict[str, str], braille_code: str) -> list[str]:
    answer = []
    has_problems = False
    for (key, val) in row.items():
        if "<math" in val:
            chars = extract_strings(val)
            if len(chars) > 1:
                has_problems = True
            answer.extend(chars)

    answer = list(set(answer))
    if has_problems:
        print(f'{line_number}: {row["Symbol Name"]}')
        for item in answer:
            print(item)
    return answer

# We have a slightly complicated match condition for the tag name to avoid "<mover...>", etc.
# We avoid capturing something "x" or "A" since that wouldn't have a code in the table
MO_CONTENTS = re.compile("<mo(?:<| .*?)>([^a-zA-Z]*?)</mo>")
MI_CONTENTS = re.compile("<mi(?:<| .*?)>([^a-zA-Z]*?)</mi>")


def extract_strings(xml: str) -> list[str]:
    answer_list = [html.unescape(s) for s in MO_CONTENTS.findall(xml)]
    if '⏜' in xml:
        print(f'DEBUGGING mo match={MO_CONTENTS.findall(string=xml)}\nFROM: val={xml}')
    if len(answer_list) == 0:
        answer_list = [html.unescape(s) for s in MI_CONTENTS.findall(xml)]
    answer_list = list(set(answer_list))
    if len(answer_list) > 1:
        print(f'extract_strings: {answer_list} from {xml}')
    return answer_list


write_unicode_file("BrailleMathCodes Repository.csv", "out", "Marburg")

