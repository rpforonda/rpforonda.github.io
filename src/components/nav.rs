use crate::data::NAME;
use leptos::prelude::*;

const LINKS: &[(&str, &str)] = &[
    ("#about", "About"),
    ("#work", "Work"),
    ("#game", "Block Coders"),
    ("#experience", "Experience"),
];

/// One span per letter for the hover roll in style.css. Screen readers skip
/// them, so the link holding them needs an aria-label with the whole text.
fn roll(text: &str) -> impl IntoView {
    let letters = text
        .chars()
        .enumerate()
        .map(|(i, letter)| {
            view! { <span style=format!("--i:{i}")>{letter.to_string()}</span> }
        })
        .collect_view();
    view! { <span class="roll" aria-hidden="true">{letters}</span> }
}

#[component]
pub fn Nav() -> impl IntoView {
    let (dark, set_dark) = signal(true);
    let (menu_open, set_menu_open) = signal(false);

    // Keep <html data-theme="..."> in sync with the toggle.
    Effect::new(move |_| {
        if let Some(root) = document().document_element() {
            let _ = root.set_attribute("data-theme", if dark.get() { "dark" } else { "light" });
        }
    });

    view! {
        <header class="nav">
            <a class="brand" href="#top" aria-label=NAME>
                <span class="brand-mark">"⟡"</span>
                {roll(NAME)}
            </a>
            <nav class:open=menu_open>
                {LINKS
                    .iter()
                    .map(|(href, label)| {
                        view! {
                            <a
                                href=*href
                                aria-label=*label
                                on:click=move |_| set_menu_open.set(false)
                            >
                                {roll(label)}
                            </a>
                        }
                    })
                    .collect_view()}
            </nav>
            <div class="nav-actions">
                <button
                    class="icon-btn"
                    aria-label="Toggle theme"
                    on:click=move |_| set_dark.update(|d| *d = !*d)
                >
                    {move || if dark.get() { "☀" } else { "☾" }}
                </button>
                <button
                    class="icon-btn menu-btn"
                    aria-label="Toggle menu"
                    on:click=move |_| set_menu_open.update(|o| *o = !*o)
                >
                    "☰"
                </button>
            </div>
        </header>
    }
}
