use leptos::prelude::*;

#[component]
pub fn ArtCard(
    title: &'static str,
    href: &'static str,
    description: &'static str,
    author: &'static str,
    date: &'static str,
) -> impl IntoView {
    let fallback_src = if href.starts_with("/public/") {
        href.to_string()
    } else {
        format!("/public{}", href)
    };
    let image_src = RwSignal::new(href.to_string());
    let on_image_error = {
        let fallback_src = fallback_src.clone();
        move |_| {
            if image_src.get_untracked() != fallback_src {
                image_src.set(fallback_src.clone());
            }
        }
    };

    view! {
        <article class="art-entry">
            <header class="header">
                <img class="art-logo" src="/image.png" alt="Profile logo" />
                <h2>{author}</h2>
                <span>{date}</span>
            </header>
            <img
                class="content-image"
                src=move || image_src.get()
                alt=description
                loading="lazy"
                decoding="async"
                on:error=on_image_error
            />
            <div class="image-text">
                <span>{title}</span>
                " - "
                <span>{description}</span>
            </div>
        </article>
    }
}
