#!/bin/bash
set -euxo pipefail
cd "$(dirname "$0")/../"

version="$1"
npm version "${version}" --git-tag-version false
python3 ./script/update_tauri_version.py "${version}" "./src-tauri/tauri.conf.json"
python3 ./script/update_cargo_version.py "${version}" "./src-tauri/Cargo.toml"
