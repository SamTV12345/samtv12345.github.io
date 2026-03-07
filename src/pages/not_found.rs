use crate::page_meta::configure_page;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    configure_page("404", "route-home");

    view! {
        <main class="not-found">
            <div>
                <h1>"Page not found"</h1>
                <A href="/" attr:class="btn btn--primary">"Back home"</A>
            </div>
        </main>
    }
}
