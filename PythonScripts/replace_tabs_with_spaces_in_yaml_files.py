import os
import argparse

def replace_tabs_with_spaces(file_path, spaces_per_tab):
    with open(file_path, 'r') as file:
        lines = file.readlines()

    updated = False
    for i, line in enumerate(lines):
        if '\t' in line:
            new_line = []
            count = 0
            for char in line:
                if char == '\t':
                    spaces = spaces_per_tab - (count % spaces_per_tab)
                    new_line.append(' ' * spaces)
                    count += spaces
                else:
                    new_line.append(char)
                    count += 1
            lines[i] = ''.join(new_line)
            updated = True

    if updated:
        print(f"Replacing tabs with spaces in file: {file_path}")
        with open(file_path, 'w') as file:
            file.writelines(lines)

def find_and_replace_in_directory(directory, spaces_per_tab):
    for root, dirs, files in os.walk(directory):
        for file in files:
            if file.endswith('.yml') or file.endswith('.yaml'):
                file_path = os.path.join(root, file)
                replace_tabs_with_spaces(file_path, spaces_per_tab)

def main():
    parser = argparse.ArgumentParser(description='Replaces tabs with spaces in .yml and .yaml files.')
    parser.add_argument('-s', '--spaces', type=int, default=4, 
                        help='Number of spaces for each tab. Default is 4.')

    args = parser.parse_args()
    
    current_directory = os.getcwd()
    find_and_replace_in_directory(current_directory, args.spaces)

if __name__ == "__main__":
    main()
