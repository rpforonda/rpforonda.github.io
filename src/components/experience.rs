use crate::data::EXPERIENCE;
use leptos::prelude::*;

#[component]
pub fn Experience() -> impl IntoView {
    view! {
        <section id="experience">
            <h2>"Experience"</h2>
            <ol class="timeline">
                {EXPERIENCE
                    .iter()
                    .map(|job| {
                        view! {
                            <li class="timeline-item">
                                <div class="timeline-head">
                                    <h3>{job.title}</h3>
                                    <span class="mono timeline-dates">{job.dates}</span>
                                </div>
                                <p class="timeline-org">
                                    <strong>{job.org}</strong>
                                    " · "
                                    {job.location}
                                </p>
                                <ul>
                                    {job.bullets.iter().map(|b| view! { <li>{*b}</li> }).collect_view()}
                                </ul>
                            </li>
                        }
                    })
                    .collect_view()}
            </ol>
        </section>
    }
}
