#!/usr/bin/env bash
set -euo pipefail

SRC="target/release/libnes_ffi.so"   # Linux; change for mac/win if needed
DST="godot_frontend/bin/libnes_ffi.so"

if [[ ! -f "$SRC" ]]; then
  echo "❌ $SRC not found. Build first: cargo build-ffi"
  exit 1
fi

mkdir -p "$(dirname "$DST")"
cp -f "$SRC" "$DST"
echo "✅ Copied $SRC → $DST"
