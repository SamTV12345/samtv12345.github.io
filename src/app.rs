use crate::pages::{ArtPage, HomePage, NotFoundPage, SocialsPage};
use leptos::prelude::*;
use leptos_router::{components::*, path};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=move || view! { <NotFoundPage /> }>
                <Route path=path!("/") view=HomePage />
                <Route path=path!("/socials") view=SocialsPage />
                <Route path=path!("/art") view=ArtPage />
            </Routes>
        </Router>
    }
}
