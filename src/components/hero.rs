use crate::data::{GITHUB, LINKEDIN, LOCATION, RESUME_REQUEST_SUBJECT, ROLE, STATS, TAGLINE};
use crate::mail::MailLink;
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
                <MailLink class="btn">"Email me"</MailLink>
                <MailLink
                    class="btn"
                    title="My résumé is shared on request, so it isn't published here"
                    query=resume_request_query()
                >
                    "Request résumé"
                </MailLink>
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

/// Pre-filled subject and body for a résumé request email. The résumé is sent
/// on request rather than published, to limit what's scraped from the site.
fn resume_request_query() -> String {
    let body =
        "Hi Roel,\n\nI'd like to request your résumé.\n\nName:\nCompany:\nRole I'm hiring for:\n";
    format!(
        "?subject={}&body={}",
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
