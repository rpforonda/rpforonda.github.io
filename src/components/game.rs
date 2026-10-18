use crate::data::GAME;
use leptos::prelude::*;

/// Featured Block Coders section. The game is a separate macroquad wasm build
/// served from play/, so it's embedded in an iframe and only loaded on click.
#[component]
pub fn FeaturedGame() -> impl IntoView {
    let (shot, set_shot) = signal(0usize);
    let (playing, set_playing) = signal(false);

    view! {
        <section id="game">
            <p class="eyebrow">"Featured project · Playable"</p>
            <h2>{GAME.title}</h2>
            <p class="sub game-pitch">{GAME.pitch}</p>
            <div class="game card">
                <div class="game-stage">
                    <Show
                        when=move || playing.get()
                        fallback=move || {
                            view! {
                                <img
                                    src=move || GAME.shots[shot.get()].0
                                    alt=move || GAME.shots[shot.get()].1
                                />
                                <button class="play-overlay" on:click=move |_| set_playing.set(true)>
                                    <span class="play-icon">"▶"</span>
                                    "Play in your browser"
                                </button>
                            }
                        }
                    >
                        <iframe
                            src=GAME.play_url
                            title="Block Coders game"
                            allow="fullscreen; autoplay"
                        ></iframe>
                    </Show>
                </div>
                <div class="thumbs">
                    {GAME
                        .shots
                        .iter()
                        .enumerate()
                        .map(|(i, (src, alt))| {
                            view! {
                                <button
                                    class="thumb"
                                    class:active=move || !playing.get() && shot.get() == i
                                    on:click=move |_| {
                                        set_playing.set(false);
                                        set_shot.set(i);
                                    }
                                >
                                    <img src=*src alt=*alt loading="lazy"/>
                                </button>
                            }
                        })
                        .collect_view()}
                </div>
                <p class="game-note">
                    "Best on a laptop or desktop. Click the game to give it keyboard focus. "
                    "Progress isn't saved in the browser version yet."
                </p>
            </div>
            <div class="game-details">
                <ul class="game-features">
                    {GAME.features.iter().map(|f| view! { <li>{*f}</li> }).collect_view()}
                </ul>
                <div>
                    <ul class="chips">
                        {GAME.stack.iter().map(|s| view! { <li>{*s}</li> }).collect_view()}
                    </ul>
                    <div class="cta-row">
                        <a class="btn primary" href=GAME.play_url target="_blank" rel="noopener">
                            "Open full screen ↗"
                        </a>
                        <a class="btn" href=GAME.repo target="_blank" rel="noopener">
                            "View code"
                        </a>
                    </div>
                </div>
            </div>
        </section>
    }
}
