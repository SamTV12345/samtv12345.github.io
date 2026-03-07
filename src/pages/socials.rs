use crate::components::{
    GitHubLogo, InstagramLogo, LinkItem, SnapchatLogo, TwitchLogo, YoutubeLogo,
};
use crate::page_meta::configure_page;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn SocialsPage() -> impl IntoView {
    configure_page("My Socials ♥", "route-socials");

    view! {
        <main class="socials-main">
            <div class="bg-orb" aria-hidden="true"></div>
            <div class="bg-grid socials-grid-mask" aria-hidden="true"></div>

            <section class="socials-shell">
                <header class="socials-header">
                    <div id="social-header-logo">
                        <img class="socials-cover" src="/bild_rose.jpg" alt="Rose cover" />
                        <img class="socials-avatar" src="/image.png" alt="samtv1998 profile" />
                    </div>
                    <h1 class="socials-title">
                        "Folge "
                        <span class="grad">"samtv1998"</span>
                    </h1>
                    <p class="socials-subtitle">"Alle Plattformen auf einen Blick"</p>
                </header>

                <nav class="socials-links" aria-label="Social links">
                    <LinkItem href="https://github.com/samtv12345" title="GitHub" body="">
                        <GitHubLogo />
                    </LinkItem>
                    <LinkItem href="https://snapchat.com/add/samtv1998" title="Snapchat" body="">
                        <SnapchatLogo />
                    </LinkItem>
                    <LinkItem href="https://instagram.com/samtv1998/" title="Instagram">
                        <InstagramLogo />
                    </LinkItem>
                    <LinkItem href="https://www.youtube.com/@samtv1998" title="YouTube">
                        <YoutubeLogo />
                    </LinkItem>
                    <LinkItem href="https://www.twitch.tv/samtv1998/" title="Twitch">
                        <TwitchLogo />
                    </LinkItem>
                </nav>

                <footer class="socials-actions">
                    <A href="/" attr:class="socials-back">
                        "← Zur Startseite"
                    </A>
                </footer>
            </section>
        </main>
    }
}
