mod components;
mod data;
mod mail;

use components::*;
use leptos::prelude::*;

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
    mount_to_body(App);
    scroll_to_hash();
}

/// The sections only exist once this app has mounted, which is after the
/// browser has already looked for a link's #section and given up. So a direct
/// link like /#game would stay at the top: jump to the section ourselves.
fn scroll_to_hash() {
    let Ok(hash) = window().location().hash() else { return };
    let Some(id) = hash.strip_prefix('#').filter(|id| !id.is_empty()) else { return };
    if let Some(el) = document().get_element_by_id(id) {
        // Instant, not the page's smooth scrolling: a long glide on load feels broken.
        let opts = web_sys::ScrollIntoViewOptions::new();
        opts.set_behavior(web_sys::ScrollBehavior::Instant);
        el.scroll_into_view_with_scroll_into_view_options(&opts);
    }
}
