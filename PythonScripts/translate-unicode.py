# Translate unicode characters into the target language
# This makes use of three sources: SRE's translations, MathPlayer's translations, and Google translate.

# The google translate is done via https://github.com/ffreemt/google-stranslate (pip install itranslate)
# from itranslate import itranslate as translate
# TRANSLATE_URL = "https://translate.google.us"

# Old -- this fails due to gender issues (e.g, try translating "the" to Spanish). People mentioned fixes, but the project seems dead
# The google translate is done via googletrans
# Note: needed to use 'pip install googletrans==4.0.0-rc1' and there is some concern this package might go away
from googletrans import Translator
GoogleTranslate = Translator(service_urls=["translate.google.us"])

# Google allows up to 500K chars translation/month, so using a key likely would be free anyway

# Got blocked trying to translated unicode-full.yaml (~8,100 text strings).
# Some sites said ~450 requests in a short time will causes a block
# Some other places said there is ~5,000 char limit to a block request.
# There are ~15 chars/request on average
# To speed things up and avoid getting blocked, two passes are taken:
# 1. Go through the document and find all strings to translate and add them to a dictionary.
# 2. Convert the dictionary to a list, break it into appropriate chunks, and translate each chunk, building up a new dictionary
# 3. Process the file and decide on using MP, SRE, or Google for the text; add comments for the others
# This results in unneeded translations because if MP and SRE agree, we shouldn't bother with the translation.
#  However, knowing what MP and SRE will use for translations requires knowing the 'ch' and that's more work.
# The solution I've adopted is a bit ugly in that the two passes have some duplication. 


def translate_char(ch:str, ignore_ch: bool, en_text: str, mathplayer: dict, sre: dict, google: dict):
    mp_trans = mathplayer[ch] if ch in mathplayer else ''
    sre_trans = sre[ch] if ch in sre else ''
    # don't bother do the translation if mp and sre agree
    google_trans = ''
    # print("mp_trans/sre_trans: '{}/{}'".format(mp_trans,sre_trans))
    if ignore_ch or mp_trans != sre_trans or mp_trans == '':
        en_text = en_text.replace("eigh", "a").replace("Eigh", "A").replace("cap", "uppercase").replace("paren", "parenthesis")
        if ignore_ch:
            mp_trans = ''
            sre_trans = ''
        if len(en_text) > 1:
            google_trans = google[en_text]
            # print("Google Translation:('{}') = '{}'".format(en_text, google_trans))
        else:
            google_trans = ch

    return (mp_trans, sre_trans, google_trans)

import re
TextToTranslate = re.compile('t: "([^"]+)"')
def translate_char_line(ch: str, line:str, mathplayer: dict, sre: dict, google: dict):
    # using a closure for this is ugly
    result = {}
    
    def do_translate_char(match_obj):
        if match_obj:
            alternatives = []
            ignore_ch = line.find('then:') >= 0   # usually an alternative to what mp and sre would say
            mp_trans, sre_trans, google_trans = translate_char(ch, ignore_ch, match_obj.group(1), mathplayer, sre, google)
            # print("ch='{}', mp/sre/google={}/{}/{}\n".format(ch, mp_trans, sre_trans, google_trans))
            translation = google_trans
            if mp_trans == sre_trans and mp_trans:  # if mp and sre match, then we don't care about google
                translation = mp_trans
            elif google_trans == mp_trans:  # google_trans is never empty, so can't match an empty mp_trans
                translation = mp_trans
                if sre_trans and sre_trans != mp_trans:
                    alternatives.append("SRE: '{}'".format(sre_trans))
            elif google_trans == sre_trans: # google_trans is never empty, so can't match an empty sre_trans
                translation = sre_trans
                if mp_trans:  # already know sre_trans != mp_trans
                    alternatives.append("MathPlayer: '{}'".format(mp_trans))
            # at this point we know none of the options match, but some can be empty
            elif sre_trans:
                translation = sre_trans
                if mp_trans:
                    alternatives.append("MathPlayer: '{}'".format(mp_trans))
                alternatives.append( "google: '{}'".format(google_trans) )
            elif mp_trans:
                translation = mp_trans
                if sre_trans:
                    alternatives.append("SRE: '{}'".format(sre_trans))
                alternatives.append( "google: '{}'".format(google_trans) )
            else:       # only translation comes from google
                translation = google_trans

            result['original'] = match_obj.group(1)
            result['translation'] = translation
            result['alternatives'] = alternatives
            return 't: "{}"'.format(translation)
        else:
            return line
    return ( line if line.lstrip().startswith('#') else TextToTranslate.sub(do_translate_char, line),  result )

# char defs take one of two forms:
# single line: - "̇": [t: "dot above embellishment"]             # 0x307
# multiple lines:
# - "Α":                                          # 0x391
#     - test: 
#         if: "($Blind or $Verbosity!='Terse')"
#         then: [t: "cap"]
#     - t: alpha

CharDefStart = re.compile('[^-]*- "([^"])"')        # we skip ranges such as A-Z
# gather lines until a char def is found
def get_next_char_def(lines: list):
    iStart = 1
    while iStart < len(lines):
        if CharDefStart.match(lines[iStart]):
            return lines[:iStart]
        iStart += 1
    return lines        # last char definition
 
def gather_words_in_char_def(lines: list, lang: str, mathplayer: dict, sre: dict, words_to_translate: set):

    def gather_words_for_text(ch: str, en_text:str, lang: str, mathplayer: dict, sre: dict, words_to_translate: set):
        mp_trans = mathplayer[ch] if ch in mathplayer else ''
        sre_trans = sre[ch] if ch in sre else ''
        # don't bother do the translation if mp and sre agree
        google_trans = ''
        # print("mp_trans/sre_trans: '{}/{}'".format(mp_trans,sre_trans))
        if mp_trans != sre_trans or mp_trans == '':     # note: ch=='' => mp_trans==''
            en_text = en_text.replace("eigh", "a").replace("Eigh", "A").replace("cap", "uppercase").replace("paren", "parenthesis")
            if len(en_text) > 1:
                words_to_translate.add(en_text)

                
    ch_match = CharDefStart.match(lines[0])
    ch = ch_match.group(1) if ch_match else ''

    for line in lines:
        en_text = TextToTranslate.search(line)  # assumes only one "t:" per line
        if en_text:
            # if "then:" is present, it is usually an alternative to what mp and sre would say
            ch_for_line = '' if line.find('then:') else ch
            gather_words_for_text(ch_for_line, en_text.group(1), lang, mathplayer, sre, words_to_translate)

    return words_to_translate

# echo lines, substituting for any "t:"
def process_char_def(lines: list, mathplayer: dict, sre: dict, google: dict, out_stream):
    match = CharDefStart.match(lines[0])
    ch = match.group(1) if match else ''
    for line in lines:
        translated_line, details = translate_char_line(ch, line, mathplayer, sre, google)
        if translated_line:
            # make comments that don't start a line mostly align
            i_comment_char = translated_line.find('#')
            if i_comment_char > 0 and translated_line.find('"#"') >= 0:
                # avoid considering '#' in the case of it being defined: - "#": [t: "number"]   
                i_comment_char = translated_line.find('#', i_comment_char+1)
            comment = ''
            if i_comment_char > 0 and not(translated_line.lstrip().startswith('#')):
                comment = translated_line[i_comment_char+1:].rstrip()
                translated_line = translated_line[:i_comment_char-1]
            if 'alternatives' in details:
                alternatives = details['alternatives']
                if details['original'] != details['translation']:
                    alternatives.insert(0, "en: '{}'".format(details['original']))
                if alternatives != []:
                    comment += '\t(' + alternatives[0]
                    for str in alternatives[1:]:
                        comment += ", " + str
                    comment += ')'
            if comment:
                translated_line = "{:<48s}\t# {}\n".format(translated_line.rstrip(), comment)
            # print("***{}\t# {}\n".format(translated_line.rstrip(), comment))
        out_stream.write(
            (translated_line if ch else line)
        )

# run over the file and figure out what words need to be translated
def collect_words_to_translate(file_to_translate: str, lang: str, mathplayer: dict, sre: dict):
    with open(file_to_translate, 'r', encoding='utf8') as in_stream:
        lines = in_stream.readlines()
        iLine = 0
        words_to_translate = set()
        while iLine < len(lines):
            char_def_lines = get_next_char_def(lines[iLine:])
            # print("\niLines={}\n{}".format(iLine, list(map(lambda l: l+"\n", char_def_lines))))
            if len(char_def_lines) == 0:
                break
            gather_words_in_char_def(char_def_lines, lang, mathplayer, sre, words_to_translate)
            iLine += len(char_def_lines)
        return words_to_translate

# break up the words into chunks to make google translate happy (and to run faster) and return a dictionary of word: translation
MAX_CHARS_IN_CHUNK = 4500
import time
def translate_words(words_to_translate, lang):
    translations = {}

    def do_translation_chunk(words: list):
        # translate doesn't handle a list properly -- use ".\n" to separate words
        word_string = ".\n".join(words)
        # chunk_translations = translate(words, from_lang='en', to_lang=lang, url=TRANSLATE_URL)
        translated_words = GoogleTranslate.translate(word_string, src='en', dest=lang).text.lower()
        translated_words = translated_words.split('.\n')
        if len(translated_words) != len(words_to_translate):
            print("\n!!!Problem in translation: size of translations ({}) differs from words to translate ({})\n".format(len(translated_words), len(words_to_translate)))
        for (orig, translation) in zip(words, translated_words):
            translations[orig] = translation

    word_list = set(words_to_translate)
    char_count = 0
    words_to_translate = []
    for word in word_list:
        words_to_translate.append(word)
        char_count += len(word)
        if char_count >= MAX_CHARS_IN_CHUNK:
            do_translation_chunk(words_to_translate)
            print("Translated {} words...".format(len(words_to_translate)))
            char_count = 0
            words_to_translate = []
            time.sleep(2)       # try to avoid google banning us
    do_translation_chunk(words_to_translate)
    return translations


def create_new_file(file_to_translate: str, output_file: str, mathplayer: dict, sre: dict, google: dict):
    with open(file_to_translate, 'r', encoding='utf8') as in_stream:
        with open(output_file, 'w', encoding='utf8') as out_stream:
            lines = in_stream.readlines()
            iLine = 0
            while iLine < len(lines):
                char_def_lines = get_next_char_def(lines[iLine:])
                # print("\niLines={}\n{}".format(iLine, list(map(lambda l: l+"\n", char_def_lines))))
                if len(char_def_lines) == 0:
                    break
                process_char_def(char_def_lines, mathplayer, sre, google, out_stream)
                iLine += len(char_def_lines)

def build_new_translation(path_to_mathcat: str, lang: str, unicode_file_name: str):
    sre = get_sre_unicode_dict(SRE_Location, lang)
    mathplayer = get_mathplayer_unicode_dict(MP_Location, lang)

    file_lang_to_translate = lang if lang=='vi' or lang=='id' else 'en'      # these are already partially translated
    file_to_translate = "{}/Rules/Languages/{}/{}.yaml".format(path_to_mathcat, file_lang_to_translate, unicode_file_name)
    words_to_translate = collect_words_to_translate(file_to_translate, lang, mathplayer, sre)
    google = translate_words(words_to_translate, lang)
    print("Translations: MathPlayer={}, SRE={}, Google={}".format(len(mathplayer), len(sre), len(google)))
    create_new_file(file_to_translate, "{}-{}.yaml".format(unicode_file_name, lang), mathplayer, sre, google)


import os
import json
def get_sre_unicode_dict(path:str, lang: str):
    try:
        dict= {}
        path += "\\" + lang + "\\" + "symbols" + "\\"
        for filename in os.listdir(path):
            with open(path+filename, 'r', encoding='utf8') as in_stream:
                # print( "\nReading file {}".format(path+filename) )
                sre_data = json.load(in_stream)
                for sre_entry in sre_data:
                    # entries we care about look like {"key": "2212", "mappings": {"default": {"default": "menos"}}}
                    if "key" in sre_entry and "default" in sre_entry["mappings"]:
                        key = chr(int(sre_entry["key"], base=16))
                        dict[key] = sre_entry["mappings"]["default"]["default"]
        return dict
    except:
        return {}

# entries we care about look like char ? (unicode == 0x2212) => string{text="menos";};
# or char ? (unicode == 0x004E) => string{text= "n"+(::target_group!="Blind" ? "" : " majuscule");};;

MP_Pattern = re.compile(r'.*?\(unicode == 0x([0-9A-Fa-f]{4,5})\).*?"([^"]+)".*?')
def get_mathplayer_unicode_dict(path: str, lang: str):
    path += "\\" + lang + "\\"
    try:
        dict= {}
        with open(path+"unicode.tdl", 'r', encoding='utf8') as in_stream:
            lines = in_stream.readlines()
            for line in lines:
                matches = MP_Pattern.match(line)
                if matches:
                    int_key = int(matches.group(1), base=16)
                    text = matches.group(2).strip()
                    # MP makes use of char in the private use area: E000—F8FF -- don't add those
                    if (int_key < 0xE000 or int_key > 0xF8FF) and text:
                        key = chr(int_key)
                        dict[key] = text
        return dict
    except:
        return {}

# for some diagnostics (from stackoverflow.com)
def dict_compare(lang: str, sre: dict, mp: dict):
    sre_keys = set(sre.keys())
    mp_keys = set(mp.keys())
    shared_keys = sre_keys.intersection(mp_keys)
    sre_only = sre_keys - mp_keys
    mp_only = mp_keys - sre_keys
    differ = {o : (sre[o], mp[o]) for o in shared_keys if sre[o] != mp[o]}
    same = set(o for o in shared_keys if sre[o] == mp[o])
    with open("diffs-{}.txt".format(lang), 'w', encoding='utf8') as out_stream:
        def print_dict(name, dict):
            out_stream.write("\n\n---{}---\n".format(name))
            for key in dict:
                out_stream.write("  {}({:0>4x})={}\n".format(key, ord(key), dict[key]))
        def print_set(name, set, orig_dict):
            out_stream.write("\n---{}---\n".format(name))
            for key in set:
                out_stream.write("  {}({:0>4x})='{}'\n".format(key, ord(key), orig_dict[key]))
        out_stream.write("sre/mp #chars={}/{}, #same={}, #differ={}, #only sre/mp={}/{}"
            .format(len(sre), len(mp), len(same), len(differ), len(sre_only), len(mp_only) ))
        print_dict("differ", differ)
        print_set("sre_only", sre_only, sre)
        print_set("mp_only", mp_only, mp)
    return (sre_only, mp_only, differ, same)


import sys
sys.stdout.reconfigure(encoding='utf-8')

# if os.path.exists("unicode.yaml"):
#   os.remove("unicode.yaml")
SRE_Location = r"C:\Dev\speech-rule-engine\mathmaps"
MP_Location = r"C:\Dev\mathplayer\EqnLib\rules\pvt"
# (sre_only, mp_only, differ, same) = dict_compare("es", sre_chars, mp_chars)
# (sre_only, mp_only, differ, same) = dict_compare("fr", get_sre_unicode_dict(SRE_Location, "fr"), get_mathplayer_unicode_dict(MP_Location, "fr"))
# (sre_only, mp_only, differ, same) = dict_compare("it", get_sre_unicode_dict(SRE_Location, "it"), get_mathplayer_unicode_dict(MP_Location, "it"))
# build_new_translation("..", "fr", "unicode")
build_new_translation("..", "vi", "unicode-full")