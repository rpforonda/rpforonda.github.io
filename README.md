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

## Make it yours

| What | Where |
|---|---|
| Name, links, stats, skills, projects, certs, flow steps | `src/data.rs` |
| Colors / theme / layout | `style.css` (CSS variables at the top) |
| Page title, SEO, Open Graph tags | `index.html` |
| Images, favicon | drop into `public/` (served at `public/...`). Use WebP (lossless for screenshots and pixel art). |
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

`play/gl.js` and `play/audio.js` are the JS loaders from the game's exact crate versions (miniquad 0.4.11, quad-snd 0.2.8).
If the game's `Cargo.lock` bumps those, copy the new files from `~/.cargo/registry/src/*/<crate>/js/`.
macroquad's all-in-one `mq_js_bundle.js` isn't used: its websocket plugin throws under `"use strict"`.

## Deploy (GitHub Pages)

1. `git init`, commit, and push to a new GitHub repo.
   - Name it `rpforonda.github.io` to serve at `https://rpforonda.github.io/`,
   - or any other name (e.g. `portfolio`) to serve at `https://rpforonda.github.io/portfolio/`.
2. In the repo: **Settings → Pages → Build and deployment → Source: GitHub Actions**.
3. Push to `main`. `.github/workflows/pages.yml` builds the game and the site, sets the right base path, and deploys.
4. Optional: add a custom domain in **Settings → Pages** and put the domain in `public/CNAME`.

All in-site links are relative (`public/...`, `play/...`), so the same build works at the root or under `/<repo>/`.

Direct links to a section (e.g. `/#game`) work: the sections only exist once the app has mounted, after the browser
has given up looking for the anchor, so `scroll_to_hash` in `src/main.rs` jumps there itself.

## Upgrading to Leptos 0.9

This code also compiles against `0.9.0-beta2`. To try it, change the `leptos` version in `Cargo.toml` to `"=0.9.0-beta2"`.
