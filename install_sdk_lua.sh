#!/usr/bin/env bash
cd "$(dirname "$0")"
set -e

if [ -z "$1" ]; then
  echo "Error: First argument should be a path where to install Fyrox Lua SDK" >&2
  exit 1
fi

if [ -e "$1" ]; then
  echo "Error: specified path points to existing file or directory. Non-existing path required" >&2
  exit 1
fi

INSTALL_DIR=$(realpath $1)

cargo build -p fyrox_lite_lua
cargo build -p fyroxed_lua

mkdir -p "$INSTALL_DIR"

cp target/debug/fyrox_lite_lua.exe $INSTALL_DIR
cp target/debug/fyroxed_lua.exe $INSTALL_DIR

echo "Fyrox Lua SDK has been installed to $INSTALL_DIR"
