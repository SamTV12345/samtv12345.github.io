use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Card(
    link: &'static str,
    href: &'static str,
    title: &'static str,
    body: &'static str,
    #[prop(optional)] target: Option<&'static str>,
) -> impl IntoView {
    let link_content = view! {
        <>
            <h2>
                {title}
                <span>{link}</span>
            </h2>
            <p>{body}</p>
        </>
    };

    view! {
        <li class="link-card">
            {match target {
                Some(target) => view! {
                    <a href=href target=target>
                        {link_content.clone()}
                    </a>
                }
                .into_any(),
                None => view! {
                    <A href=href>
                        {link_content}
                    </A>
                }
                .into_any(),
            }}
        </li>
    }
}
