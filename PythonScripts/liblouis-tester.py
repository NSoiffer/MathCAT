# test all files in the dir to see if the braille translation of the input matches the expected output
# usage:
#   python liblouis-tests.py [-f] test_dir [result_file]
#   -f: if given, only failures are shown
#   test_dir: the directory to test (will recurse)
#   result_file: if given, the results are written there; otherwise written to stdout
#
# the result for each test is the filename followed by "succeeded" or "failed"
# in case of failure, the result and the expected result are also shown
from pathlib import Path
# if out_file is empty, then it prints to stdout
def test(rel_path: str, out_stream, failure_only=False):
    path = Path.joinpath(Path.cwd(), rel_path)
    result = run_test(path, failure_only)
    for one_test in result:
        out_stream.write(one_test)
        out_stream.write('\n')

def run_test(abs_path: Path, failure_only: bool):
    result = []
    if abs_path.is_dir():
        file_names = [file.name for file in abs_path.iterdir()]
        if 'input.xml' in file_names and 'expected.txt' in file_names:
            error_msg = run_single_test(abs_path)
            if len(error_msg) == 0:
                if not(failure_only):
                    result.append("{} succeeded".format(abs_path))
            else:
                result.append("{} failed:\n{}".format(abs_path, error_msg))
        else:   # recurse in the subdirs
            for sub_path in abs_path.glob('*'):
                result += run_test(Path.joinpath(abs_path,sub_path), failure_only)
    # if not a dir, ignore it
    return result

# this should only be called in a directory with the required files
# returns the result of the diff (success is an empty string)
import subprocess
def run_single_test(test_dir: Path):
    # print("run_single_test for {}".format(test_dir))
    liblouisutdml_ini_file = str(find_file(test_dir, 'liblouisutdml.ini'))
    input_file = str(Path.joinpath(test_dir, 'input.xml'))
    # print("ini_file: '{}', input_file: '{}'".format(liblouisutdml_ini_file, input_file))
    file2brl_result = subprocess.run(['file2brl.exe', '-l', '-w', '.', '-f',  liblouisutdml_ini_file, input_file], capture_output=True, encoding="utf-8")
    if file2brl_result.stderr:
        return "***file2braille error on file {}: {}".format(str(test_dir), file2brl_result.stderr)
    braille = file2brl_result.stdout.rstrip().replace(' ', '⠀') # replace ascii space with Unicode space
    expected = Path.joinpath(test_dir, 'expected.txt').read_text(encoding='utf-8').rstrip()
    if expected.isascii():
        expected = ascii_to_unicode(expected)

    return '' if braille == expected else "  Expected: '{}'\n  Returned: '{}'".format(expected, braille)

# look in the current dir and in ancestors for the file name.
# return that absolute path
def find_file(start_dir: Path, file_name: str):
    target_path = Path.joinpath(start_dir, file_name)
    if Path.exists(target_path):
        return target_path
    parent_path = start_dir.parent
    if parent_path == start_dir:
        raise RuntimeError("Can't find file {}".format(file_name))
    else:
        return find_file(parent_path, file_name)

ASCII_TO_UNICODE = "⠀⠮⠐⠼⠫⠩⠯⠄⠷⠾⠡⠬⠠⠤⠨⠌⠴⠂⠆⠒⠲⠢⠖⠶⠦⠔⠱⠰⠣⠿⠜⠹⠈⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚⠅⠇⠍⠝⠕⠏⠟⠗⠎⠞⠥⠧⠺⠭⠽⠵⠪⠳⠻⠘⠸"
def ascii_to_unicode(ascii: str):
    result = "";
    ascii = ascii.upper()
    for ch in ascii:
        i = ord(ch) - 32
        result += ASCII_TO_UNICODE[i]
    return result

import sys
opts = [opt for opt in sys.argv[1:] if opt.startswith("-")]
args = [arg for arg in sys.argv[1:] if not arg.startswith("-")]
if len(args) == 1 or len(args)==2:
    sys.stdout.reconfigure(encoding='utf-8')
    out_stream = open(args[1], "w", encoding="utf-8") if len(args)==2 else sys.stdout
    test(args[0], out_stream, failure_only= len(opts)==1 and "-f" in opts)
    if len(args)==2:
        out_stream.close()
else:
    print("Usage: python {} [-f] inputDir [outFile]".format(Path(sys.argv[0]).name))
