#!/bin/bash
set -e

# Find the built executable in the release directory
BIN_PATH=$(find ../../target -type f -path "*/release/edgebar.exe" | head -n 1)

if [ -z "$BIN_PATH" ]; then
  echo "Executable not found!"
  exit 1
fi

echo "Compressing $BIN_PATH with UPX..."
upx --best --lzma "$BIN_PATH"