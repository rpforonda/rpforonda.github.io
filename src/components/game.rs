use crate::data::GAME;
use leptos::prelude::*;

/// Phones are too small for the inline 16:10 embed, so they get the full-page
/// game instead, which lays itself out for the phone (upright or sideways).
fn is_phone() -> bool {
    window()
        .match_media(
            "(pointer: coarse) and (max-width: 760px), (pointer: coarse) and (max-height: 500px)",
        )
        .ok()
        .flatten()
        .is_some_and(|mq| mq.matches())
}

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
                                <button
                                    class="play-overlay"
                                    on:click=move |_| {
                                        if is_phone() {
                                            let _ = window().location().set_href(GAME.play_url);
                                        } else {
                                            set_playing.set(true);
                                        }
                                    }
                                >
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
                    "Progress is saved only in this browser on this device. Nothing is sent anywhere. "
                    "Plays on laptops, tablets and phones, held either way."
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
