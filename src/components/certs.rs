use crate::data::CERTS;
use leptos::prelude::*;

#[component]
pub fn Certs() -> impl IntoView {
    view! {
        <section id="certs">
            <h2>"Certifications"</h2>
            <div class="grid three">
                {CERTS
                    .iter()
                    .map(|c| {
                        view! {
                            <a class="card cert" href=c.url target="_blank" rel="noopener">
                                <span class="cert-code mono">{c.code}</span>
                                <span>{c.name}</span>
                                <span class="cert-meta">{c.issuer} " · " {c.status}</span>
                            </a>
                        }
                    })
                    .collect_view()}
            </div>
        </section>
    }
}
