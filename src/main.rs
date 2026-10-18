mod components;
mod data;

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
            <FlowDemo/>
            <Certs/>
        </main>
        <Footer/>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
