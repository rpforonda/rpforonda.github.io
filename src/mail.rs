use crate::data::{EMAIL_DOMAIN, EMAIL_USER};
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;

/// The address written out as "name at domain", for visitors without a mail
/// app to read and type. Spam bots mostly look for `name@domain`.
pub fn written_out() -> String {
    format!("{EMAIL_USER} at {EMAIL_DOMAIN}")
}

/// An email link that only becomes a real `mailto:` link once the visitor
/// points at, focuses or touches it. Bots that render the page rarely do, but
/// people can still copy the link, open it in a new tab or click it.
/// `subject` and `body` pre-fill the message when given.
#[component]
pub fn MailLink(
    #[prop(optional)] class: &'static str,
    #[prop(optional)] title: &'static str,
    #[prop(optional)] subject: &'static str,
    #[prop(optional)] body: &'static str,
    children: Children,
) -> impl IntoView {
    let mailto = StoredValue::new(mailto(subject, body));
    // Set on the element itself rather than through a signal: Leptos applies
    // signal changes a moment later, so a click straight after focus would
    // still follow "#".
    let reveal = move |ev: &web_sys::Event| {
        if let Some(a) = link(ev) {
            let _ = a.set_attribute("href", &mailto.get_value());
        }
    };
    view! {
        <a
            class=class
            title=(!title.is_empty()).then_some(title)
            href="#"
            on:pointerenter=move |ev| reveal(&ev)
            on:focus=move |ev| reveal(&ev)
            on:click=move |ev| {
                // Nothing revealed the link first (unusual): open mail directly.
                if link(&ev).and_then(|a| a.get_attribute("href")).as_deref() == Some("#") {
                    ev.prevent_default();
                    let _ = window().location().set_href(&mailto.get_value());
                }
            }
        >
            {children()}
        </a>
    }
}

fn link(ev: &web_sys::Event) -> Option<web_sys::Element> {
    ev.current_target()?.dyn_into().ok()
}

fn mailto(subject: &str, body: &str) -> String {
    let mut url = format!("mailto:{EMAIL_USER}@{EMAIL_DOMAIN}");
    let fields = [("subject", subject), ("body", body)];
    for (i, (name, value)) in fields.iter().filter(|(_, v)| !v.is_empty()).enumerate() {
        let sep = if i == 0 { '?' } else { '&' };
        // RFC 6068: line breaks in a mailto: body are CRLF.
        let value = value.replace("\r\n", "\n").replace('\n', "\r\n");
        url.push_str(&format!("{sep}{name}={}", encode(&value)));
    }
    url
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
