import xml.etree.ElementTree as ET


# FIX: manually added POSTFIX priority 25 for '.,;:?'
def convert_xml_entities(in_file: str, out_obj):
    tree = ET.parse(in_file)
    root = tree.getroot()
    all_chars = root.find("charlist")
    if all_chars is None:
        print(f"Didn't find 'charlist' in {in_file}")
        return
    for char in all_chars:
        count = 0
        for op_dict in char.findall("operator-dictionary"):
            # "+" => OperatorInfo{ op_type: OperatorTypes::INFIX, priority: 280,
            #	next: &Some(OperatorInfo{ op_type: OperatorTypes::PREFIX, priority: 690, next: &None }) },
            if count == 0:
                out_obj.write(f"\t\"{convert_to_char(char.get('id', 'error'))}\" => ")
            else:
                out_obj.write("\n\t\t\tnext: &Some( ")
            out_obj.write(('OperatorInfo{ '
                           f'op_type: OperatorTypes::{compute_form(op_dict.get("form", ""), op_dict.get("fence", ""))}, '
                           f'priority: {op_dict.get("priority", "")}, '))

            count += 1
        if count > 0:
            out_obj.write("next: &None ")
            if count > 1:
                while count > 1:
                    out_obj.write("} )")
                    count -= 1
            out_obj.write("},\n")


def convert_to_char(str: str) -> str:
    # str is 'Uddddd' or 'Uddddd-ddddd'
    str = str.split("U")[1]  # strip leading 'U'
    answer = ""
    for char_str in str.split("-"):
        # FIX: need to add backslash is str becomes ""
        ch = chr(int(char_str, base=16))
        if (ch == '"' or ch == '\\'):
            answer += "\\"
        answer += ch

    return answer


def compute_form(form: str, fence: str):
    if fence is None or fence == "false":
        return form.upper()
    # fence must be true
    if form == "prefix":
        return "LEFT_FENCE"
    elif form == "postfix":
        return "RIGHT_FENCE"
    else:
        return form.upper()     # shouldn't happen


def add_wrappers(in_file: str, out_file: str):
    with open(out_file, 'w', encoding="utf8") as out_obj:
        out_obj.write("phf_map! {\n")
        convert_xml_entities(in_file, out_obj)
        # the following are not in the operator dictionary and should get added "manually"
        # mod should take its priority from '%'s priority
        out_obj.write("\n\t// the following are not in the operator dictionary and are added \"manually\"\n")
        out_obj.write("\t\"mod\" => OperatorInfo{ op_type: OperatorTypes::INFIX, priority: 800, next: &None },\n")
        out_obj.write("}\n")

def get_binary_ops(in_file: str):
    tree = ET.parse(in_file)
    root = tree.getroot()
    all_chars = root.find("charlist")
    if all_chars is None:
        print(f"Didn't find 'charlist' in {in_file}")
        return
    binary_ops = []
    double_char_binary_ops = []
    for ch in all_chars:
        if ch.get('type', '') == 'binaryop':
            unicode_str = ch.get('id', '')
            if unicode_str.find('-') == -1:
                binary_ops.append(chr(int(unicode_str[1:], 16)))
            else:
                parts = unicode_str.split('-')
                double_char_binary_ops.append((chr(int(parts[0][1:], 16)), parts[1]))
    print(binary_ops)
    print(f'Found {len(binary_ops)} binary operators')
    print(f'Other binary chars: {double_char_binary_ops}')


# add_wrappers("c:\\dev\\mathml-refresh\\xml-entities\\unicode.xml", "out.rs")
get_binary_ops("c:\\dev\\mathml-refresh\\xml-entities\\unicode.xml")
