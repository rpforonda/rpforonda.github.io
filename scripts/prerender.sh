#!/usr/bin/env bash
# Bakes a rendered copy of the page into the built index.html. Search engines
# and link previews, which don't run the app, then see the content, and
# visitors see text before the app loads. src/main.rs swaps the copy for the
# live app.
# Usage: scripts/prerender.sh [dist] [base]   (base is trunk's --public-url)
# An enhancement only: if there's no Chrome or the app doesn't render, it warns
# and leaves the page as it was, so a deploy never fails because of it.
set -euo pipefail
dist="${1:-dist}"
base="${2:-/}"

warn() { echo "::warning::prerender skipped: $1"; exit 0; }
chrome="${CHROME:-$(command -v google-chrome || command -v chromium || true)}"
[[ -n "$chrome" ]] || warn "no Chrome or Chromium found"

# Serve dist at the same path the site lives at, so its links resolve.
tmp="$(mktemp -d)"
if [[ "$base" == "/" ]]; then
  serve="$(realpath "$dist")"
else
  mkdir -p "$tmp/root$(dirname "${base%/}")"
  ln -s "$(realpath "$dist")" "$tmp/root${base%/}"
  serve="$tmp/root"
fi
python3 -m http.server 8765 --bind 127.0.0.1 --directory "$serve" >/dev/null 2>&1 &
server=$!
trap 'kill "$server" 2>/dev/null; rm -rf "$tmp"' EXIT
url="http://127.0.0.1:8765$base"
curl -sf -o /dev/null --retry 30 --retry-connrefused --retry-delay 1 "$url" || warn "local server didn't start"

# --virtual-time-budget lets the app download, start and render before the dump.
"$chrome" --headless=new --disable-gpu --no-sandbox --hide-scrollbars \
  --virtual-time-budget=15000 --dump-dom "$url" > "$tmp/rendered.html" 2>/dev/null || warn "Chrome failed"

python3 - "$tmp/rendered.html" "$dist/index.html" <<'EOF' || warn "the app didn't render"
import re, sys
rendered, index = open(sys.argv[1], encoding="utf-8").read(), sys.argv[2]
body = re.search(r"<body[^>]*>(.*)</body>", rendered, re.S)
copy = re.sub(r"<script\b.*?</script>", "", body.group(1) if body else "", flags=re.S).strip()
if "<main" not in copy:
    sys.exit(1)
html = open(index, encoding="utf-8").read()
if "<body></body>" not in html:
    sys.exit(1)
html = html.replace("<body></body>", f'<body><div id="prerendered">{copy}</div></body>', 1)
open(index, "w", encoding="utf-8").write(html)
print(f"prerendered {len(copy)} characters into {index}")
EOF
