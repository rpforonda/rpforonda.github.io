use crate::data::ABOUT;
use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <section id="about">
            <h2>"About"</h2>
            <div class="about panel">
                {ABOUT.iter().map(|p| view! { <p>{*p}</p> }).collect_view()}
            </div>
        </section>
    }
}
