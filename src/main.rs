mod components;
mod data;
mod mail;

use components::*;
use leptos::prelude::*;
use leptos::wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlElement, ScrollBehavior, ScrollToOptions};

/// html's scroll-padding-top in style.css: where a section's top lands, just
/// under the nav.
const SCROLL_PADDING: f64 = 80.0;

#[component]
fn App() -> impl IntoView {
    view! {
        <Nav/>
        <main>
            <Hero/>
            <About/>
            <Skills/>
            <Projects/>
            <FeaturedGame/>
            <Experience/>
            <Certs/>
        </main>
        <Footer/>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    // The deploy bakes a rendered copy of the page into index.html for search
    // engines and link previews (scripts/prerender.sh). Swap it for the live app.
    if let Some(copy) = document().get_element_by_id("prerendered") {
        copy.remove();
    }
    mount_to_body(App);
    scroll_to_hash();
    follow_section_links();
    track_button_glow();
}

/// The glow inside a button sits under the cursor: keep the cursor's spot on
/// the button in --x and --y for style.css. One listener for the whole page.
fn track_button_glow() {
    let _ = window_event_listener(leptos::ev::pointermove, |e| {
        let Some(btn) = e
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
            .and_then(|el| el.closest(".btn").ok().flatten())
            .and_then(|el| el.dyn_into::<HtmlElement>().ok())
        else {
            return;
        };
        let rect = btn.get_bounding_client_rect();
        let style = btn.style();
        let _ = style.set_property("--x", &format!("{}px", f64::from(e.client_x()) - rect.left()));
        let _ = style.set_property("--y", &format!("{}px", f64::from(e.client_y()) - rect.top()));
    });
}

/// The sections only exist once this app has mounted, which is after the
/// browser has already looked for a link's #section and given up. So a direct
/// link like /#game would stay at the top: jump to the section ourselves.
fn scroll_to_hash() {
    let Ok(hash) = window().location().hash() else { return };
    let Some(id) = hash.strip_prefix('#').filter(|id| !id.is_empty()) else { return };
    // Instant, not the page's smooth scrolling: a long glide on load feels broken.
    scroll_to_section(id, ScrollBehavior::Instant);
}

/// In-page links (#about and so on) scroll to the section's top. Left to the
/// browser, they'd aim at where the section appears on screen, which the scroll
/// animations move: one you've scrolled past sits a screen lower until you come
/// back to it, so the jump would stop halfway, mid-animation.
fn follow_section_links() {
    let _ = window_event_listener(leptos::ev::click, |e| {
        // Opening in a new tab or window stays the browser's job.
        if e.default_prevented() || e.button() != 0 || e.ctrl_key() || e.meta_key() || e.shift_key() || e.alt_key() {
            return;
        }
        let Some(link) = e
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
            .and_then(|el| el.closest("a[href^='#']").ok().flatten())
        else {
            return;
        };
        let href = link.get_attribute("href").unwrap_or_default();
        if scroll_to_section(&href[1..], ScrollBehavior::Auto) {
            e.prevent_default();
            if window().location().hash().ok().as_deref() != Some(href.as_str()) {
                let _ = window().history().and_then(|h| h.push_state_with_url(&JsValue::NULL, "", Some(&href)));
            }
        }
    });
}

/// Scrolls the section's top to just under the nav, going by its place in the
/// layout, which transforms don't change. Auto follows the page's smooth
/// scrolling (off for reduced motion). False if there's no such section.
fn scroll_to_section(id: &str, behavior: ScrollBehavior) -> bool {
    let Some(el) = document().get_element_by_id(id).and_then(|e| e.dyn_into::<HtmlElement>().ok()) else {
        return false;
    };
    let mut top = 0;
    let mut node = Some(el);
    while let Some(n) = node {
        top += n.offset_top();
        node = n.offset_parent().and_then(|p| p.dyn_into::<HtmlElement>().ok());
    }
    let opts = ScrollToOptions::new();
    opts.set_top(f64::from(top) - SCROLL_PADDING);
    opts.set_behavior(behavior);
    window().scroll_to_with_scroll_to_options(&opts);
    true
}
