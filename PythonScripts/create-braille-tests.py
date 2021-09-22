# create braille tests from SRE braille tests

from bs4 import BeautifulSoup

def create_test_from_SRE_test(in_file: str, out_file):
    with open(in_file, encoding='utf8') as _in_stream:
        with open(out_file, 'w', encoding='utf8') as out_stream:
            out_stream.write('use crate::common::*;\n\n')
            file_contents = BeautifulSoup(_in_stream, features="html.parser")
            for row in file_contents.find_all('tr'):
                cols = row.find_all('td')
                generate_test(out_stream, cols[0].get_text(), cols[1].math, cols[2].get_text())

def generate_test(out_stream, test_name: str, mathml, braille: str):
    del mathml["xmlns"]         # shorter MathML

    # SRE adds class attrs as part of its parsing, but those cause spelling warnings, so remove them
    for tag in mathml.find_all(['mi', 'mn', 'mo', 'ms', 'mrow']):
            del tag["class"]
            del tag["data-mjx-texclass"]
    mathml_str = str(mathml).replace('"', '\\"')
    if len(mathml_str) > 80:
        # prettify() creates too many lines, so we do a few simple linebreaks
        mathml_str = mathml_str.replace('<mrow>', '\n        <mrow>')
        mathml_str = mathml_str.replace('<mfrac>', '\n        <mfrac>')
        mathml_str = mathml_str.replace('<msup>', '\n        <msup>')
        mathml_str = mathml_str.replace('<mroot>', '\n        <mroot>')
        mathml_str = mathml_str.replace('<msqrt>', '\n        <msqrt>')
    out_stream.write('#[test]\n')
    out_stream.write('fn test_{}() {}\n'.format(test_name, "{"))
    out_stream.write('    let expr = "{}";\n'.format(mathml_str))
    out_stream.write('    test_braille("Nemeth", expr, "{}");\n'.format(braille))
    out_stream.write('{}\n\n'.format("}"))


create_test_from_SRE_test("C:/Dev/speech-rule-engine/sre-tests/output/nemeth/NemethBase.html", "SRE_NemethBase.rs")
create_test_from_SRE_test("C:/Dev/speech-rule-engine/sre-tests/output/nemeth/Nemeth72.html", "SRE_Nemeth72.rs")
create_test_from_SRE_test("C:/Dev/speech-rule-engine/sre-tests/output/nemeth/AataNemeth.html", "AataNemeth.rs")
