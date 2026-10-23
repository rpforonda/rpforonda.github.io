#!/usr/bin/env bash
# Builds Block Coders (github.com/rpforonda/block-coders) to wasm and drops it in play/.
# Usage: scripts/build-game.sh [path-to-block-coders-checkout]
# Without an argument, it clones/updates the repo into .game-src/.
set -euo pipefail
cd "$(dirname "$0")/.."

src="${1:-.game-src}"
if [[ -z "${1:-}" ]]; then
  if [[ -d "$src/.git" ]]; then
    git -C "$src" pull --ff-only --quiet
  else
    git clone --depth 1 --quiet https://github.com/rpforonda/block-coders "$src"
  fi
fi

# macroquad's audio/GL functions are provided by mq_js_bundle.js at runtime,
# so the linker must leave them as imports.
RUSTFLAGS="-C link-arg=--allow-undefined" \
  cargo build --release --target wasm32-unknown-unknown --manifest-path "$src/Cargo.toml"

# Ask cargo where it built: CARGO_TARGET_DIR or a build.target-dir setting moves
# the output, and a hardcoded $src/target would silently ship an older build.
target_dir="$(cargo metadata --format-version 1 --no-deps --manifest-path "$src/Cargo.toml" \
  | python3 -c 'import json, sys; print(json.load(sys.stdin)["target_directory"])')"
out="$target_dir/wasm32-unknown-unknown/release/block-coders.wasm"
if command -v wasm-opt >/dev/null; then
  wasm-opt -Oz --enable-bulk-memory --enable-nontrapping-float-to-int --enable-sign-ext \
    --enable-mutable-globals "$out" -o play/block-coders.wasm
else
  cp "$out" play/block-coders.wasm
fi
# The game's own JS plugins (e.g. web/storage.js for browser saves) ship with it,
# so the plugin always matches the Rust side that calls it.
if [[ -d "$src/web" ]]; then
  cp "$src"/web/*.js play/
fi
# Fonts built into the game ship with their license (SIL OFL), next to the game.
if compgen -G "$src/assets/fonts/OFL-*.txt" >/dev/null; then
  mkdir -p play/licenses
  cp "$src"/assets/fonts/OFL-*.txt play/licenses/
fi
ls -lh play/block-coders.wasm
