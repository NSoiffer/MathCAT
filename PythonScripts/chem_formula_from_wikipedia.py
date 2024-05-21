from bs4 import BeautifulSoup
import re

SingleLetterSingleElementFormulae = re.compile(r"^(?P<single>[A-Z](_\d)?(\^\d?[+\-−])?)$")
SingleLetterDoubleElementFormulae = re.compile(r"^(?P<first>[A-Z](_\d)?(\d?[+\-−])?)(?P<second>[A-Z](_\d)?(\^\d?[+\-−])?)$")


def create_formulae_from_wikipedia_page(in_file: str, out_file):
    with open(in_file, encoding='utf8') as _in_stream:
        with open(out_file, 'w', encoding='utf8') as out_stream:
            file_contents = BeautifulSoup(_in_stream, features="html.parser")
            formulaeSet = set()
            for row in file_contents.find_all('tr'):
                cols = row.find_all('td')
                if len(cols) == 3 and cols[0].get_text():
                    result = add_formula_to_set(formulaeSet, cols[0].decode_contents())

            result = ''
            for formula in sorted(formulaeSet):
                entry = '"{}", '.format(formula)
                if len(result) + len(entry) > 78:
                    out_stream.write(result + '\n')
                    result = ''
                else:
                    result += entry

            if len(result) > 0:
                out_stream.write(result)


def add_formula_to_set(formulaeSet, data):
    # the data isn't clean -- do some cleanup
    data = data.replace("<sub>", "_").replace("</sub>", "").replace("<sup>", "^").replace("</sup>", "")
    data = data.strip()
    oneElement = SingleLetterSingleElementFormulae.match(data)
    if not (oneElement is None):
        formulaeSet.add(oneElement.group("single"))
    else:
        twoElements = SingleLetterDoubleElementFormulae.match(data)
        if not (twoElements is None):
            formulaeSet.add(twoElements.group("first") + twoElements.group("second"))
            formulaeSet.add(twoElements.group("second") + twoElements.group("first"))


def create_ions_from_wikipedia_page(in_file: str, out_file):
    with open(in_file, encoding='utf8') as _in_stream:
        with open(out_file, 'w', encoding='utf8') as out_stream:
            file_contents = BeautifulSoup(_in_stream, features="html.parser")
            formulaeSet = set()
            for ion in file_contents.find_all(class_='chemf'):
                result = add_single_letter_ions_to_set(formulaeSet, ion.decode_contents())

            result = ''
            for formula in sorted(formulaeSet):
                entry = '"{}", '.format(formula)
                if len(result) + len(entry) > 79:
                    out_stream.write(result + '\n')
                    result = ''
                else:
                    result += entry

            if len(result) > 0:
                out_stream.write(result)


BOTH_SCRIPTS = re.compile(r'([^<]+)<span class="template-chem2-su"><span>(\d?[+−])</span><span>(\d)</span></span>')


def add_single_letter_ions_to_set(formulaeSet, data):
    # the data isn't clean -- do some cleanup
    data = data.replace('<sub>', "_").replace('<sub class="template-chem2-sub">', "_").replace('</sub>', "") \
                .replace('<sup>', "^").replace('<sup class="template-chem2-sup">', "^").replace('</sup>', "")
    bothScripts = BOTH_SCRIPTS.match(data)
    if not (bothScripts is None):
        data = "{}_{}^{}".format(bothScripts.group(1), bothScripts.group(3), bothScripts.group(2))
    else:
        bothScripts = BOTH_SCRIPTS.match(data)

    data = data.strip()
    if not (any(ch.islower() for ch in data)):
        formulaeSet.add(data)


# create_formulae_from_wikipedia_page("wikipedia-chemical_formulae.html", "chem_formula.txt")
create_ions_from_wikipedia_page("wikipedia-ions.html", "chem_ions.txt")
