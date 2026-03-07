use leptos::prelude::*;

#[component]
pub fn LinkItem(
    href: &'static str,
    title: &'static str,
    #[prop(optional)] body: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    view! {
        <a class="link-item" href=href target="_blank" rel="noopener noreferrer" aria-label=title>
            <h2>
                {children()}
                <span class="link-item__title">{title}</span>
            </h2>
            {body
                .and_then(|value| (!value.is_empty()).then_some(value))
                .map(|value| view! { <p>{value}</p> })}
        </a>
    }
}
