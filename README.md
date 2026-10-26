# Portfolio (Rust + Leptos + WASM)

A single-page portfolio built with [Leptos](https://leptos.dev) 0.8 (client-side rendered) and bundled with [Trunk](https://trunkrs.dev).

## Run locally

```bash
# one-time setup (Arch: or `sudo pacman -S rust-wasm` if you use the system rust package)
rustup target add wasm32-unknown-unknown
cargo install --locked trunk

trunk serve            # http://localhost:8080, hot reload
trunk build --release  # static output in dist/
```

The only crates are `leptos` and `web-sys` (with just the browser APIs the site calls). Small jobs that a crate would
usually do are a few lines here instead: panics go to the browser console from `main`, and `src/mail.rs` builds and
encodes the `mailto:` link itself.

### Build output

`trunk build --release` gives a ~82 KB (gzipped) `.wasm`, ~5.5 KB of JS glue and ~6.6 KB of CSS. The release profile
in `Cargo.toml` optimizes for size (`opt-level = "z"`, LTO, one codegen unit, `panic = "abort"`), Trunk runs
`wasm-opt -Oz` (`data-wasm-opt="z"` in `index.html`; it must be on `PATH`), and `Trunk.toml` minifies the HTML and JS.
`style.css` opts out of minifying (`data-no-minify`): the minifier rewrites `max-width` media queries as range syntax
(`width<=760px`), which Safari before 16.4 ignores, so older iPhones would lose the mobile layout.

The fonts are self-hosted, Latin subset only (`public/fonts/`), since the content is English. If you add accented
names, add the matching `latin-ext` files from the same Fontsource packages and their `@font-face` rules.

## Make it yours

| What | Where |
|---|---|
| Name, links, stats, skills, projects, certs, game copy and screenshots | `src/data.rs` |
| Colors / theme / layout | `style.css` (CSS variables at the top) |
| Page title, SEO, Open Graph tags | `index.html` (link-preview image: `public/og-card.png`, 1200×630: a screenshot of the hero at that window size, saved as a 256-colour PNG, which keeps it ~25 KB) |
| Images | drop into `public/` (served at `public/...`). Use WebP; for game screenshots see below. |
| Resume | not published: the hero's **Request resume** button opens a pre-filled email. Source and PDFs live outside this repo in `~/Work/resume/` (`build.sh`). |
| Section order | `App` in `src/main.rs` |

Each section is its own component in `src/components/`.

## Block Coders (the embedded game)

The game is its own repo, [rpforonda/block-coders](https://github.com/rpforonda/block-coders), built with macroquad.
macroquad doesn't use wasm-bindgen, so it can't be a Leptos component. Instead it's a standalone page in `play/`
that the site embeds in an iframe and only loads when someone clicks **Play**.

```bash
scripts/build-game.sh            # clones/updates the game into .game-src/ and writes play/block-coders.wasm
scripts/build-game.sh ../block-coders   # or build from an existing checkout
trunk serve                      # the game is at /play/index.html
```

`build-game.sh` asks `cargo metadata` where the build went, so it also works with `CARGO_TARGET_DIR` or a
`build.target-dir` setting (a hardcoded `target/` path would quietly ship an older build).

On phones, **Play** opens the game's own page (`play/index.html`), which fills the screen. The game lays itself out for
phones held upright or sideways. Where the browser can go full screen (Android, iPad; not iPhone Safari), a small start
card offers **Play full screen** once per tab; after that nothing covers the game, since its own buttons sit in every
corner.

The game's fonts (Atkinson Hyperlegible and its Mono) are built into the wasm. `build-game.sh` copies their SIL Open
Font License texts to `play/licenses/`, so the license ships with the game.

Keyboard users can Tab into the game and back out again. miniquad's `gl.js` cancels Tab on the canvas, which would
trap focus inside the iframe; the game never uses Tab, so `play/index.html` stops Tab before it reaches the canvas.

### Screenshots

The game section shows a big screenshot with a row of thumbnails, one per entry in `GAME.shots` (`src/data.rs`). Each
`shot!("name", "caption")` points at three files in `public/block-coders/`: `name.webp` (1280 wide, lossless),
`name-640.webp` and `name-320.webp`. The big picture lists the first two in `srcset`, so phones and small windows load
the 640 copy; the thumbnails only ever load the 320 copy. Everything below the fold loads lazily.

To retake them, capture 1280×800 PNGs with the game's screenshot autopilot (debug builds only), then convert them:

```bash
cd ../block-coders
BC_SHOT=/tmp/shots BC_LEVEL=36 cargo run   # level index from 0; saves 3-intro … 7-finished PNGs, then quits
cd -
cp /tmp/shots/6-running.png /tmp/if-else-level.png
scripts/screenshots.sh /tmp/if-else-level.png   # writes the three WebP files (needs ImageMagick 7)
```

The autopilot also takes `BC_TOUR=1` with `BC_INPUT="x,y,move|down|up|shot;..."` to click through the guided tour, and
its `2d-select.png` is the level select. See `BC_SHOT` in the game's `src/main.rs`.

`play/gl.js` and `play/audio.js` are the JS loaders from the game's exact crate versions (miniquad 0.4.11, quad-snd 0.2.8).
If the game's `Cargo.lock` bumps those, copy the new files from `~/.cargo/registry/src/*/<crate>/js/`.
macroquad's all-in-one `mq_js_bundle.js` isn't used: its websocket plugin throws under `"use strict"`.

Sound on iPad and iPhone needs three things, all in `play/index.html` (not in `audio.js`, so it can still be copied as is):
iOS mutes Web Audio in Silent Mode, so the page asks for a `"playback"` audio session (`navigator.audioSession`, iOS 17
and later) and, on older iOS, loops a short silent `<audio>` after the first tap; and until the game's AudioContext is
running, every tap resumes it and starts a one-sample silent sound on it (`audio.js` only tries on the first
`touchstart`, which Safari doesn't accept). The game's Sound button still turns sound off.

## Deploy (GitHub Pages)

1. `git init`, commit, and push to a new GitHub repo.
   - Name it `rpforonda.github.io` to serve at `https://rpforonda.github.io/`,
   - or any other name (e.g. `portfolio`) to serve at `https://rpforonda.github.io/portfolio/`.
2. In the repo: **Settings → Pages → Build and deployment → Source: GitHub Actions**.
3. Push to `main`. `.github/workflows/pages.yml` builds the game and the site, sets the right base path, and deploys.
   After the build, `scripts/prerender.sh` loads the page once in headless Chrome and bakes the rendered HTML into
   `index.html`. Search engines and link previews, which don't run the app, see the content, and visitors see text
   before the app loads. The app swaps the copy for itself when it mounts. If Chrome is missing or the page doesn't
   render, the script warns and the deploy goes ahead without it.
4. Optional: add a custom domain in **Settings → Pages** and put the domain in `public/CNAME`.

All in-site links are relative (`public/...`, `play/...`), so the same build works at the root or under `/<repo>/`.

Direct links to a section (e.g. `/#game`) work. The browser jumps to the baked-in copy. Once the app mounts and replaces
it, `scroll_to_hash` in `src/main.rs` jumps there again, which also covers builds where the prerender was skipped.

## Upgrading to Leptos 0.9

This code also compiles against `0.9.0-rc0` (checked 2026-10-25; `0.9.0-beta2` no longer builds with current
dependencies). To try it, change the `leptos` version in `Cargo.toml` to `"=0.9.0-rc0"`.
