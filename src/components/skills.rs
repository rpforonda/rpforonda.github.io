use crate::data::SKILLS;
use leptos::prelude::*;

#[component]
pub fn Skills() -> impl IntoView {
    view! {
        <section id="skills">
            <h2>"What I work with"</h2>
            <div class="grid four">
                {SKILLS
                    .iter()
                    .map(|g| {
                        view! {
                            <article class="card">
                                <div class="card-icon">{g.icon}</div>
                                <h3>{g.title}</h3>
                                <ul class="chips">
                                    {g.items.iter().map(|i| view! { <li>{*i}</li> }).collect_view()}
                                </ul>
                            </article>
                        }
                    })
                    .collect_view()}
            </div>
        </section>
    }
}
