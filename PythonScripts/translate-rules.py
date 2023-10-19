"""
Translate the English rule files (but not the unicode files) to the target language.
This is done with the function build_all_translations().

The unicode files are not built here because they are large enough to seem to occasionally run into hiccups.

See the end of this file how this is used (typically change 'language' and just run the file)
"""
# Translate text in rules into the target language

# The google translate is done via https://github.com/ffreemt/google-stranslate (pip install itranslate)
# from itranslate import itranslate as translate
# TRANSLATE_URL = "https://translate.google.us"

# The google translate is done via googletrans
# Note: needed to use 'pip install googletrans==4.0.0-rc1' and there is some concern this package might go away
from googletrans import Translator
GoogleTranslate = Translator(service_urls=["translate.google.us"])

# Google allows up to 500K chars translation/month, so using a key likely would be free anyway

# Unlike the unicode file, the rule files don't have a lot of text.
# 
# To speed things up and avoid getting blocked, two passes are taken:
# 1. For each file, we gather all the text into a list that has "phrase(..'xxx'...)". We prepend <line#>:: to the phrase string.
# 2. Turn the list into a string with separators, translate it, and reconvert to a list
# 3. Reread the file replacing translations (we know the line number) and writing it out

import re
PhraseToTranslate = re.compile(r'phrase\(([^)]+)\)')

# run over the file and figure out what words need to be translated
def collect_phrases_to_translate(file_to_translate: str) -> list[str]:
    with open(file_to_translate, 'r', encoding='utf8') as in_stream:
        translations = []
        for line in in_stream:
            phrase = PhraseToTranslate.search(line)
            if phrase:
                translations.append(phrase.group(1))
        return translations

# break up the words into chunks to make google translate happy (and to run faster) and return a dictionary of word: translation
MAX_CHARS_IN_CHUNK = 4500  # 4500 sometimes failed (language code "no")

# try to avoid google banning us
TIMEOUT = 2
import time

def translate_phrases(phrases_to_translate: list[str], lang) -> list[str]:
    if lang=='nb' or lang=='nn':
        lang = 'no'  # google doesn't know those variants, but SRE uses them

    def do_translation_chunk(phrases: list[str]):
        if len(phrases) == 0:
            return phrases             # file with no "phrase(...)"
        # translate doesn't handle a list properly -- use ".\n" to separate phrases
        phrases_string = ".\n".join(phrases)
        # print("***Phrases to translate: {}\n".format(phrases))
        translated_phrases_str = GoogleTranslate.translate(phrases_string, src='en', dest=lang).text.lower()
        translated_phrases_str = translated_phrases_str.replace('"', "'").replace("“", "'").replace("”", "'")    # google occasionally changes quotes
        translated_phrases_str = translated_phrases_str.replace("、", ",")   # Chinese comma
        translated_phrases_list = translated_phrases_str.split('.\n')
        if len(translated_phrases_list) != len(phrases):
            print("\n!!!Problem in translation: size of translations ({}) differs from phrases to translate ({})\n".format(len(translated_phrases_list), len(phrases)))
            print("English phrases: {}\n".format(phrases))
            print("Truncated translated phrases: {}\n".format(translated_phrases_list))
            # The Finnish translation (at least) for some reason has a few failures where ".\n" is only "\n" (and translation failed)
            # We try a last attempt by deleting the '.' and splitting at the newline
            print("Retrying by assuming '.' is missing...")
            translated_phrases_list = translated_phrases_str.replace('.','').split('\n')
            if len(translated_phrases_list) != len(phrases):
                print("!!!***Retry failed: size of translations ({}) differs from phrases to translate ({})\n".format(len(translated_phrases_list), len(phrases)))
            print("Phrases to translate:\n{}".format(list(phrases)))
            print("Translations:\n{}".format(list(translated_phrases_list)))
        return translated_phrases_list

    translations = []
    char_count = 0
    phrases_chunks_to_translate = []
    for phrase in phrases_to_translate:
        phrases_chunks_to_translate.append(phrase)
        char_count += len(phrase)
        if char_count >= MAX_CHARS_IN_CHUNK:
            print("char_count={}", char_count)
            translations += do_translation_chunk(phrases_chunks_to_translate)
            print("Translated {} phrases...".format(len(phrases_chunks_to_translate)))
            char_count = 0
            phrases_chunks_to_translate = []
            time.sleep(TIMEOUT)       # try to avoid google banning us
    return translations + do_translation_chunk(phrases_chunks_to_translate)

TargetWord = re.compile(r"'([^']+)'")
TextString = re.compile(r't: "([^"]+)"')
def substitute_in_translated_word(line, translated_phrase) -> str:
    # print("original phrase:   {}".format(TextToTranslate.search(line).group(1)))
    # print("translated phrase: {}".format(translated_phrase))
    target_words = TargetWord.search(translated_phrase)
    new_line = line
    if target_words:
        replacement = 't: "' + target_words.group(1) + '"'    # add the surrounding context back
        new_line = TextString.sub(replacement, line)
        # print("fixed line: {}".format(new_line))
    else:
        print("ERROR: failed to find quoted part in translation \"{}\"\n   original line: {}".format(translated_phrase, line))
    return new_line


import os
import sys
from os.path import isfile
sys.stdout.reconfigure(encoding='utf-8')

def create_new_file(file_to_translate: str, output_file: str, translations: list[str]) -> None:
    with open(file_to_translate, 'r', encoding='utf8') as in_stream:
        with open(output_file, 'w', encoding='utf8') as out_stream:
            iTranslation = 0
            for line in in_stream:
                if PhraseToTranslate.search(line):
                    out_stream.write(substitute_in_translated_word(line, translations[iTranslation]))
                    iTranslation += 1
                else:
                    out_stream.write(line)

def build_new_translation(path_to_mathcat: str, lang: str, rule_file_name: str) -> None:
    print("build_new_translation: rule_file_name=", rule_file_name)
    file_to_translate = "{}/Rules/Languages/en/{}".format(path_to_mathcat, rule_file_name)
    phrases_to_translate = collect_phrases_to_translate(file_to_translate)
    translations = translate_phrases(phrases_to_translate, lang)
    create_new_file(file_to_translate, os.path.join(lang, rule_file_name), translations)

def build_all_translations(path_to_mathcat: str, lang: str, subdir="") -> None:
    dir_to_translate = os.path.join(path_to_mathcat, "Rules", "Languages", "en", subdir)
    entries = os.listdir(dir_to_translate)
    for entry in entries:
        if os.path.isdir(os.path.join(dir_to_translate, entry)):
            build_all_translations(path_to_mathcat, lang, os.path.join(subdir, entry))
        elif entry.endswith('.yaml') and not(entry=="definitions.yaml" or entry=="unicode.yaml" or entry=="unicode-full.yaml"):
            # the excluded files are built in translate-unicode.py and need some manual checking so not included here
            build_new_translation(path_to_mathcat, lang, os.path.join(subdir, entry))            



language = 'zh-cn'
if not os.path.exists(language):
    os.makedirs(language)
if not os.path.exists(language+"/SharedRules"):
    os.makedirs(language+"/SharedRules")
# build_new_translation("..", language, "SharedRules/general.yaml")
build_all_translations("..", language)
