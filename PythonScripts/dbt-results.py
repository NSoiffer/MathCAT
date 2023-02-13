from pathlib import Path
from typing import TextIO

# The input looks like (with potentially blank lines also [should be filtered out for get_name_dbt_mathcat]):
# #[test]: test_000
#     TE/.-#JJJ
#   _% R _L S (MOD N) _:
# -----------
# ⠀⠀⠀⠀⠞⠑⠌⠨⠤⠼⠚⠚⠚
# ⠀⠀⠸⠩⠀⠗⠀⠸⠇⠀⠎⠀⠷⠍⠕⠙⠀⠝⠾⠀⠸⠱
# -----------
# MCAT: ⠗⠀⠸⠇⠀⠎⠷⠍⠕⠙⠀⠝⠾

def get_one_test_result(file: TextIO):
    def find_start(file: TextIO):
        while True:
            line = file.readline()
            if not line:
                return []
            elif line.find('#[test]:') >= 0:
                return [line]
    
    def find_stop(file: TextIO):
        answer = []
        while True:
            line = file.readline()
            if not line:
                raise Exception("Didn't find matching test_braille(...) line")
            elif line.find('MCAT:') >= 0:
                answer.append(line)
                return answer
            elif line.strip() != '':       # skip blank lines
                answer.append(line)
        
    answer = find_start(file)   # first line
    if answer == []:
        return answer
    return answer + find_stop(file)

import re
NAME_MATCH = re.compile(r'#\[test\]: (\w+)')
DBT_MATCH = re.compile(r'⠸⠩⠀([⠀-⣿]+)⠀⠸⠱')   # unicode braille chars
MATHCAT_MATCH = re.compile(r'MCAT:_([⠀-⣿]+)')   # unicode braille chars
def get_name_dbt_mathcat(lines: list[str]):
    name = NAME_MATCH.search(lines[0]).group(1)
    # print("***name={}, lines[-1]={}MATHCAT_MATCH.search(lines[-1])={}".format(name, lines[-1], MATHCAT_MATCH.search(lines[-1])))
    dbt = DBT_MATCH.search(lines[5]).group(1)
    mathcat = MATHCAT_MATCH.search(lines[-1]).group(1)
    # print("name={}, dbt={}, mathcat={}".format(name, dbt, mathcat))
    return (name, dbt, mathcat)

# returns true if the test is a success
def report_results(out_stream, name: str, dbt: str, mathcat: str):
    if dbt == mathcat:
        out_stream.write("{}: succeeded\n".format(name))
        return True
    else:
        out_stream.write("{}: failed\n  Duxbury: '{}'\n  MathCAT: '{}'\n".format(name, dbt, mathcat))
        return False

def generate_test_results(path: str, out_stream, failure_only=False):
    with Path(path).open('r', encoding='utf-8') as source:
        n_successes = 0
        n_tests = 0
        n_exceptions = 0
        while True:
            list_of_lines = get_one_test_result(source)
            if list_of_lines == []:
                break
            n_tests += 1
            try:
                (name, dbt, mathcat) = get_name_dbt_mathcat(list_of_lines)
                if report_results(out_stream, name, dbt, mathcat):
                    n_successes += 1
            except:
                out_stream.write("***{} test exception -- couldn't do comparison\n".format(NAME_MATCH.search(list_of_lines[0]).group(1)))
                n_exceptions += 1
        
        out_stream.write("#tests={}, success rate={:.1f}%, (#successes={}, #failures={}), #exceptions={}\n"
                .format(n_tests, 100.0*n_successes/(n_tests-n_exceptions), n_successes, n_tests-n_successes-n_exceptions, n_exceptions))

import sys
opts = [opt for opt in sys.argv[1:] if opt.startswith("-")]
args = [arg for arg in sys.argv[1:] if not arg.startswith("-")]
if len(args) == 1 or len(args)==2:
    sys.stdout.reconfigure(encoding='utf-8')
    out_stream = open(args[1], "w", encoding="utf-8") if len(args)==2 else sys.stdout
    generate_test_results(args[0], out_stream, failure_only= len(opts)==1 and "-f" in opts)
    if len(args)==2:
        out_stream.close()
else:
    print("Usage: python {} [-f] dbtFile [outFile]".format(Path(sys.argv[0]).name))
