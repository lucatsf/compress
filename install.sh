#!/bin/bash

set -e

PROGRAM_NAME="compress"
INSTALL_DIR="/opt/$PROGRAM_NAME"
BIN_PATH="./$PROGRAM_NAME"

if [ ! -f "$BIN_PATH" ]; then
    echo "Error: $BIN_PATH binary not found."
    exit 1
fi

if [ ! -d "$INSTALL_DIR" ]; then
    sudo mkdir -p "$INSTALL_DIR"
fi

sudo cp "$BIN_PATH" "$INSTALL_DIR/"

sudo chmod +x "$INSTALL_DIR/$PROGRAM_NAME"

ALIAS_CMD="alias compress='$INSTALL_DIR/$PROGRAM_NAME'"
if ! grep -q "alias compress=" ~/.bashrc; then
    echo "$ALIAS_CMD" >> ~/.bashrc
    echo "Alias 'compress' adicionado ao ~/.bashrc"
else
    echo "Alias 'compress' já existe no ~/.bashrc"
fi

source ~/.bashrc

echo "Complete installation."
