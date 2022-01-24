import xml
import codecs
from xml.etree.ElementTree import parse

# FIX: manually added POSTFIX priority 25 for '.,;:?'
def convert_xml_entities(in_file, out_obj):
    import xml.etree.ElementTree as ET
    tree = ET.parse(in_file)
    root = tree.getroot()
    all_chars = root.find("charlist")
    for char in all_chars:
        count = 0
        for op_dict in char.findall("operator-dictionary"):
            # 	"+" => OperatorInfo{ op_type: OperatorTypes::INFIX, priority: 280,
			#	next: &Some(OperatorInfo{ op_type: OperatorTypes::PREFIX, priority: 690, next: &None }) },
            if count == 0:
                out_obj.write("\t\"{}\" => ".format( convert_to_char(char.get("id")) ) )
            else:
                out_obj.write("\n\t\t\tnext: &Some( ")
            out_obj.write("OperatorInfo{} op_type: OperatorTypes::{}, priority: {}, ".format(
                    "{",
                    compute_form(op_dict.get("form"), op_dict.get("fence")),
                    op_dict.get("priority") ))
            count += 1
        if count > 0:
            out_obj.write("next: &None ")
            if count > 1:
                while count > 1:
                    out_obj.write("} )")
                    count -= 1
            out_obj.write("},\n")


def convert_to_char(str: str):
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
    if fence == None or fence=="false":
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


add_wrappers("c:\\dev\\mathml-refresh\\xml-entities\\unicode.xml", "out.rs")