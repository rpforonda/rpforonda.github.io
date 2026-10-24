use crate::data::{GITHUB, LINKEDIN, NAME};
use crate::mail::{self, MailLink};
use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer>
            <p>"© 2026 " {NAME} " · Built with Rust, Leptos & WebAssembly"</p>
            <p>
                <MailLink>{mail::written_out()}</MailLink>
                " · "
                <a href=GITHUB target="_blank" rel="noopener">"GitHub"</a>
                " · "
                <a href=LINKEDIN target="_blank" rel="noopener">"LinkedIn"</a>
            </p>
        </footer>
    }
}
