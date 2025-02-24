import re
import sys
import os


def extract_expr_and_fn(file_path):
    expr_list = []
    with open(file_path, 'r', encoding='utf-8') as file:
        content = file.read()

    # Use regex to find all function names starting with "fn " and multi-line expressions starting with "expr ="
    function_names = re.findall(r'fn\s+(\w+)\s*\(\)', content)
    expressions = re.findall(r'expr\s*=\s*"(.*?)"', content, re.DOTALL)

    # Clean up the extracted expressions to remove any extra newlines
    expressions = [expr.replace('\n', ' ').strip() for expr in expressions]

    for fn_name, expr in zip(function_names, expressions):
        expr_list.append((fn_name, expr))

    return expr_list


def write_to_html(expr_list, output_file):
    with open(output_file, 'w', encoding='utf-8') as file:
        file.write("<html><body>\n")
        file.write("<h1>Extracted Functions and Expressions</h1>\n")
        file.write("<ul>\n")
        for fn_name, expr in expr_list:
            file.write(f"<li>{fn_name}: {expr}</li>\n")
        file.write("</ul>\n")
        file.write("</body></html>")


def process_directory(directory_path):
    expr_list = []
    for root, _, files in os.walk(directory_path):
        for file in files:
            file_path = os.path.join(root, file)
            expr_list.extend(extract_expr_and_fn(file_path))
    return expr_list


if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: python script.py <input_path> <output_file>")
        sys.exit(1)

    input_path = sys.argv[1]
    output_file = sys.argv[2]

    if os.path.isdir(input_path):
        expr_list = process_directory(input_path)
    elif os.path.isfile(input_path):
        expr_list = extract_expr_and_fn(input_path)
    else:
        print("Invalid input path")
        sys.exit(1)

    write_to_html(expr_list, output_file)
    print(f"Extracted functions and expressions have been written to {output_file}")
