use crate::pages::{
    ArtPage, B250ePage, DatenschutzPage, HomePage, ImpressumPage, NotFoundPage, SocialsPage,
};
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
                <Route path=path!("/b250e") view=B250ePage />
                <Route path=path!("/impressum") view=ImpressumPage />
                <Route path=path!("/datenschutz") view=DatenschutzPage />
            </Routes>
        </Router>
    }
}
