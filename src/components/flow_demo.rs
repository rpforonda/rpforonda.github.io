use crate::data::FLOW_STEPS;
use leptos::prelude::*;

/// Clickable walkthrough of a real-world Power Automate flow.
#[component]
pub fn FlowDemo() -> impl IntoView {
    let (active, set_active) = signal(0usize);

    view! {
        <section id="flow">
            <h2>"Anatomy of a flow"</h2>
            <p class="sub">"Click a step to see how I build production-grade automations."</p>
            <div class="flow">
                {FLOW_STEPS
                    .iter()
                    .enumerate()
                    .map(|(i, step)| {
                        view! {
                            <button
                                class="flow-step"
                                class:active=move || active.get() == i
                                class:done=move || i < active.get()
                                on:click=move |_| set_active.set(i)
                            >
                                <span class="flow-icon">{step.icon}</span>
                                <span>{step.title}</span>
                            </button>
                        }
                    })
                    .collect_view()}
            </div>
            <div class="flow-detail card">
                <h3>{move || FLOW_STEPS[active.get()].title}</h3>
                <p>{move || FLOW_STEPS[active.get()].detail}</p>
                <div class="flow-nav">
                    <button
                        class="btn"
                        disabled=move || active.get() == 0
                        on:click=move |_| set_active.update(|a| *a = a.saturating_sub(1))
                    >
                        "← Prev"
                    </button>
                    <span class="mono">{move || format!("{}/{}", active.get() + 1, FLOW_STEPS.len())}</span>
                    <button
                        class="btn primary"
                        disabled=move || active.get() + 1 == FLOW_STEPS.len()
                        on:click=move |_| set_active.update(|a| *a = (*a + 1).min(FLOW_STEPS.len() - 1))
                    >
                        "Next →"
                    </button>
                </div>
            </div>
        </section>
    }
}
