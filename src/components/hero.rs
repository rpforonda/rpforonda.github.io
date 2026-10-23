use crate::data::{
    EMAIL, GITHUB, LINKEDIN, LOCATION, RESUME_REQUEST_SUBJECT, ROLE, STATS, TAGLINE,
};
use leptos::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section id="top" class="hero">
            <p class="eyebrow">{LOCATION} " · Open to work"</p>
            <h1>{ROLE}</h1>
            <p class="lead">{TAGLINE}</p>
            <div class="cta-row">
                <a class="btn primary" href="#work">"See my work"</a>
                <a class="btn" href=format!("mailto:{EMAIL}")>"Email me"</a>
                <a
                    class="btn"
                    href=resume_request_link()
                    title="My résumé is shared on request, so it isn't published here"
                >
                    "Request résumé"
                </a>
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

/// `mailto:` link with a pre-filled subject and body. The résumé is sent on
/// request rather than published, to limit what's scraped from the site.
fn resume_request_link() -> String {
    let body =
        "Hi Roel,\n\nI'd like to request your résumé.\n\nName:\nCompany:\nRole I'm hiring for:\n";
    format!(
        "mailto:{EMAIL}?subject={}&body={}",
        encode(RESUME_REQUEST_SUBJECT),
        encode(body)
    )
}

fn encode(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}
