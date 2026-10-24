use crate::data::{EMAIL_DOMAIN, EMAIL_USER};
use leptos::prelude::*;

/// Opens the visitor's mail app on a new message to the contact address.
/// `query` is empty or a `?subject=…&body=…` string.
pub fn compose(query: &str) {
    let _ = window()
        .location()
        .set_href(&format!("mailto:{EMAIL_USER}@{EMAIL_DOMAIN}{query}"));
}
