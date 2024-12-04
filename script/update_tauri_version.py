#!/usr/bin/env python3

import sys
import re
from typing import List
from pathlib import Path

def read_file(file_path: Path) -> List[str]:
    try:
        with open(file_path, 'r', encoding='utf-8') as file:
            return file.readlines()
    except FileNotFoundError:
        print(f"Error: The file {file_path} was not found.")
        sys.exit(1)
    except IOError:
        print(f"Error: Could not read the file {file_path}.")
        sys.exit(1)

def write_file(file_path: Path, lines: List[str]) -> None:
    try:
        with open(file_path, 'w', encoding='utf-8') as file:
            file.writelines(lines)
    except IOError:
        print(f"Error: Could not write to the file {file_path}.")
        sys.exit(1)

def update_version(lines: List[str], new_version: str) -> List[str]:
    in_package_section = False
    version_updated = False
    pattern = re.compile(r'^version\s*=\s*".*"$')

    def process_line(line: str) -> str:
        nonlocal in_package_section, version_updated

        if line.strip() == "[package]":
            in_package_section = True
        elif line.strip().startswith("[") and in_package_section:
            in_package_section = False

        if in_package_section and pattern.search(line):
            version_updated = True
            return f'version = "{new_version}"\n'
        
        return line

    updated_lines = list(map(process_line, lines))

    if not version_updated:
        print("Error: No version entry was updated. Please ensure the [package] section exists and contains a version entry.")
        sys.exit(1)

    return updated_lines

def main(new_version: str, file_path: Path) -> None:
    lines = read_file(file_path)
    updated_lines = update_version(lines, new_version)
    write_file(file_path, updated_lines)
    print(f"Version updated successfully to {new_version}")

if __name__ == '__main__':
    if len(sys.argv) != 3:
        print("Usage: python update_version.py <new_version> <file_path>")
        sys.exit(1)

    new_version = sys.argv[1]
    file_path = Path(sys.argv[2]).absolute().resolve(strict=True)
    main(new_version, file_path)
