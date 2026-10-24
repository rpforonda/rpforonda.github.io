use crate::data::{EMAIL_DOMAIN, EMAIL_USER};
use leptos::prelude::*;

/// The address written out as "name at domain", for visitors without a mail
/// app to read and type. Spam bots mostly look for `name@domain`.
pub fn written_out() -> String {
    format!("{EMAIL_USER} at {EMAIL_DOMAIN}")
}

/// An email link that only becomes a real `mailto:` link once the visitor
/// points at, focuses or touches it. Bots that render the page rarely do, but
/// people can still copy the link, open it in a new tab or click it.
/// `query` is empty or a `?subject=…&body=…` string.
#[component]
pub fn MailLink(
    #[prop(optional)] class: &'static str,
    #[prop(optional)] title: &'static str,
    #[prop(optional, into)] query: String,
    children: Children,
) -> impl IntoView {
    let mailto = format!("mailto:{EMAIL_USER}@{EMAIL_DOMAIN}{query}");
    let (href, set_href) = signal(String::from("#"));
    let reveal = {
        let mailto = mailto.clone();
        move || set_href.set(mailto.clone())
    };
    let (on_pointer, on_focus) = (reveal.clone(), reveal);
    view! {
        <a
            class=class
            title=(!title.is_empty()).then_some(title)
            href=href
            on:pointerenter=move |_| on_pointer()
            on:focus=move |_| on_focus()
            on:click=move |ev| {
                // Nothing revealed the link first (unusual): open mail directly.
                if href.get_untracked() == "#" {
                    ev.prevent_default();
                    let _ = window().location().set_href(&mailto);
                }
            }
        >
            {children()}
        </a>
    }
}
