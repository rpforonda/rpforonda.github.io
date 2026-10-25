#!/usr/bin/env bash
# Turns full-size game captures into the screenshots the site serves, in three
# widths for srcset: <name>.webp (1280, lossless, crisp pixel art at full size),
# <name>-640.webp and <name>-320.webp (lossy; scaled down, lossless buys nothing).
# Usage: scripts/screenshots.sh capture.png...   (1280x800 PNGs; <name> is the file's name)
# Needs ImageMagick 7 with WebP. Captures come from the game's screenshot autopilot
# (BC_SHOT in block-coders' src/main.rs).
set -euo pipefail
out="$(dirname "$0")/../public/block-coders"
for png in "$@"; do
  name="$(basename "${png%.*}")"
  magick "$png" -quality 100 -define webp:lossless=true -define webp:method=6 "$out/$name.webp"
  for w in 640 320; do
    magick "$png" -resize "${w}x" -quality 85 -define webp:method=6 "$out/$name-$w.webp"
  done
  ls -l "$out/$name"*.webp
done
