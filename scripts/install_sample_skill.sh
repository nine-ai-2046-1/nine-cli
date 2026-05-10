#!/bin/sh
# 安裝 sample skill 到 $HOME/.nine-cli/skills/hello
set -e
DEST="$HOME/.nine-cli/skills/hello/bin"
mkdir -p "$DEST"
cp "$(pwd)/examples/skills/hello/bin/run" "$DEST/run"
chmod +x "$DEST/run"
echo "sample skill installed to $DEST/run"
