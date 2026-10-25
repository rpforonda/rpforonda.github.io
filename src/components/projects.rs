use crate::data::{Category, Project, PROJECTS};
use leptos::prelude::*;

#[component]
pub fn Projects() -> impl IntoView {
    // None = show everything
    let (filter, set_filter) = signal::<Option<Category>>(None);

    let visible = move || {
        PROJECTS
            .iter()
            .filter(|p| filter.get().is_none_or(|c| c == p.category))
            .collect::<Vec<_>>()
    };

    view! {
        <section id="work">
            <h2>"Selected work"</h2>
            <p class="sub">"Problem → solution → measurable result."</p>
            <div class="filters" role="tablist">
                <button class:active=move || filter.get().is_none() on:click=move |_| set_filter.set(None)>
                    "All"
                </button>
                {Category::ALL
                    .iter()
                    .filter(|&&c| PROJECTS.iter().any(|p| p.category == c))
                    .map(|&c| {
                        view! {
                            <button
                                class:active=move || filter.get() == Some(c)
                                on:click=move |_| set_filter.set(Some(c))
                            >
                                {c.label()}
                            </button>
                        }
                    })
                    .collect_view()}
            </div>
            <div class="grid three">
                <For each=visible key=|p| p.id let:project>
                    <ProjectCard project/>
                </For>
            </div>
        </section>
    }
}

#[component]
fn ProjectCard(project: &'static Project) -> impl IntoView {
    view! {
        <article class="card project">
            <span class=format!("tag {}", project.category.class())>{project.category.label()}</span>
            <h3>{project.title}</h3>
            <dl>
                <dt>"Problem"</dt>
                <dd>{project.problem}</dd>
                <dt>"Solution"</dt>
                <dd>{project.solution}</dd>
            </dl>
            <p class="result">"↗ " {project.result}</p>
            <ul class="chips small">
                {project.stack.iter().map(|s| view! { <li>{*s}</li> }).collect_view()}
            </ul>
            <div class="card-links">
                {project
                    .links
                    .iter()
                    .map(|(label, href)| {
                        view! {
                            <a href=*href target="_blank" rel="noopener">
                                {*label} " →"
                            </a>
                        }
                    })
                    .collect_view()}
            </div>
        </article>
    }
}
