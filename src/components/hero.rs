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
                    subject=RESUME_REQUEST_SUBJECT
                    body=RESUME_REQUEST_BODY
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

/// Pre-filled body for a résumé request email. The résumé is sent on request
/// rather than published, to limit what's scraped from the site.
const RESUME_REQUEST_BODY: &str =
    "Hi Roel,\n\nI'd like to request your résumé.\n\nName:\nCompany:\nRole I'm hiring for:\n";
