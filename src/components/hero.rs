use crate::data::{EMAIL, GITHUB, LINKEDIN, LOCATION, RESUME_URL, ROLE, STATS, TAGLINE};
use leptos::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section id="top" class="hero">
            <p class="eyebrow">{LOCATION} " · Open to work"</p>
            <h1 class="gradient-last">{ROLE}</h1>
            <p class="lead">{TAGLINE}</p>
            <div class="cta-row">
                <a class="btn primary" href="#work">"See my work"</a>
                <a class="btn" href=format!("mailto:{EMAIL}")>"Email me"</a>
                <a class="btn" href=RESUME_URL target="_blank">"Résumé"</a>
                <a class="btn ghost" href=GITHUB target="_blank" rel="noopener">"GitHub"</a>
                <a class="btn ghost" href=LINKEDIN target="_blank" rel="noopener">"LinkedIn"</a>
            </div>
            <ul class="stats">
                {STATS
                    .iter()
                    .map(|s| {
                        view! {
                            <li>
                                <strong>{s.value}</strong>
                                <span>{s.label}</span>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </section>
    }
}
