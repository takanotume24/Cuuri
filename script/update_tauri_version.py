import json
import argparse
import sys
from pathlib import Path

def update_version_in_config(file_path: Path, new_version: str) -> None:
    # Load the existing JSON configuration
    with open(file_path, 'r', encoding='utf-8') as file:
        config = json.load(file)
    
    # Update the version number
    current_version = config.get("version", "0.0.0")
    
    if new_version == current_version:
        print(f"No change in version. The current version is already {new_version}.")
        sys.exit(1)

    config["version"] = new_version
    
    # Write the updated configuration back to the file
    with open(file_path, 'w', encoding='utf-8') as file:
        json.dump(config, file, indent=2)

    print(f"Updated version from {current_version} to {new_version}")

def main() -> None:
    # Set up argument parser
    parser = argparse.ArgumentParser(description="Update version in Tauri configuration file.")
    parser.add_argument("new_version", help="New version to set.")
    parser.add_argument("file_path", help="Path to the Tauri configuration JSON file.")
    
    # Parse arguments
    args = parser.parse_args()
    
    file_path = Path(args.file_path).absolute().resolve(strict=True)
    # Update version in the configuration file
    update_version_in_config(file_path, args.new_version)

if __name__ == "__main__":
    main()
