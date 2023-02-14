from pathlib import Path
from typing import TextIO

def generate_single_test(path: Path, name: str, input: str, expected: str):
    # make directory, then add input.xml and expected.txt
    dir = Path.joinpath(path, name+".test")
    dir.mkdir(parents=True, exist_ok=True)
    with Path.joinpath(dir, 'input.xml').open('w', encoding='utf-8') as f:
        f.write('<?xml version="1.0" encoding="UTF-8"?>\n')
        f.write(input)
        f.write('\n')
    with Path.joinpath(dir, 'expected.txt').open('w', encoding='utf-8') as f:
        f.write(expected)
        f.write('\n')
    
# this is kind of hacky, but good enough...
# find the first line that has "fn " and then scan to get the line with "test_braille"
# return a list of the lines (or an empty list if none)
#
# Here's an example:
# #[test]
# fn comma_ellipsis_in_sub_79_b_5() {
#     let expr = "<math> <msub><mi>P</mi>
#         <mrow><msub><mi>n</mi><mn>1</mn></msub>
#           <mo>,</mo>
#           <msub><mi>n</mi><mn>2</mn></msub>
#           <mo>,</mo><mo>&#x2026;</mo>
#           </mrow></msub></math>";
#     test_braille("Nemeth", expr, "⠠⠏⠰⠝⠰⠰⠂⠰⠪⠝⠰⠰⠆⠰⠪⠀⠄⠄⠄");
# }

def get_expr_test_string(file: TextIO):
    def find_start(file: TextIO):
        while True:
            line = file.readline()
            if not line:
                return []
            elif line.find('fn ') >= 0:
                return [line]
    
    def find_stop(file: TextIO):
        answer = []
        while True:
            line = file.readline()
            if not line:
                raise Exception("Didn't find matching test_braille(...) line")
            elif line.find('test_braille(') >= 0:
                answer.append(line)
                return answer
            elif not(line.lstrip().startswith('//')):       # skip comments
                answer.append(line)
        
    answer = find_start(file)   # first line
    if answer == []:
        return answer
    return answer + find_stop(file)

# first line has 'fn xxx_yyy {'
# second line has 'let expr = " ...'
# penultimate line has '...";'
# last line has 'test_braille('
# returns the tuple of strings (input, expected)
import re
NAME_MATCH = re.compile(r'fn (\w+)')
BRAILLE_MATCH = re.compile(r'"([⠀-⣿]+)"')   # unicode braille chars
def get_name_input_and_expected(lines: list[str]):
    name = NAME_MATCH.search(lines[0]).group(1)
    string_start = lines[1].find('"') + 1
    input = lines[1][string_start:] + ''.join([line for line in lines[2:-1]])
    input = input.rstrip().rstrip(';"')
    expected = BRAILLE_MATCH.search(lines[-1]).group(1)
    return (name, input, expected)

def generate_tests(rust_test: str, out_dir: str):
    out_dir = Path(out_dir)
    with Path(rust_test).open('r', encoding='utf-8') as source:
        while True:
            list_of_lines = get_expr_test_string(source)
            if list_of_lines == []:
                break
            (name, input, expected) = get_name_input_and_expected(list_of_lines)
            generate_single_test(out_dir, name, input, expected)

import sys
# opts = [opt for opt in sys.argv[1:] if opt.startswith("-")]
args = [arg for arg in sys.argv[1:] if not arg.startswith("-")]
if len(args)==2:
    generate_tests(args[0], args[1])
else:
    print("Usage: python {} rust_test_file output_directory".format(Path(sys.argv[0]).name))
