use crate::components::Card;
use crate::page_meta::configure_page;
use js_sys::Date;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn HomePage() -> impl IntoView {
    configure_page("Samuel · Portfolio", "route-home");
    let year = Date::new_0().get_full_year() as i32;

    view! {
        <main class="hero">
            <div class="bg-orb" aria-hidden="true"></div>
            <div class="bg-grid" aria-hidden="true"></div>

            <header class="hero__header">
                <img src="/face.jpg" alt="Samuel" class="avatar" />
                <h1 class="hero__title">
                    "Hi, ich bin "
                    <span class="grad">"Samuel"</span>
                </h1>
                <p class="hero__tagline">"Digital Artist · Illustrator · Content Creator"</p>
                <div class="hero__cta">
                    <A href="/socials" attr:class="btn btn--primary">"Socials"</A>
                    <A href="/art" attr:class="btn btn--ghost">"Meine Artworks"</A>
                </div>
            </header>

            <section class="showcase">
                <h2 class="section-title">"Highlights"</h2>
                <ul role="list" class="card-grid">
                    <Card
                        link="🔗"
                        href="/socials"
                        title="Socials"
                        body="Folg mir auf meinen Plattformen, schreib mir für Kollabs & Aufträge."
                    />
                    <Card
                        link="🎨"
                        href="/art"
                        title="Portfolio"
                        body="Entdecke meine neuesten Illustrationen, Skizzen und WIPs."
                    />
                    <Card
                        link="📝"
                        href="https://www.linkedin.com/in/samuel-schwanzer-733204195/"
                        title="Kontakt"
                        body="Direkt per LinkedIn kontaktieren."
                        target="_blank"
                    />
                </ul>
            </section>

            <section class="marquee" aria-label="Scrolling artwork previews">
                <div class="marquee__inner" aria-hidden="true">
                    <div class="marquee__track">
                        <img src="/fortnitelama.png" alt="Artwork preview 1" />
                        <img src="/pikachu.png" alt="Artwork preview 2" />
                        <img src="/spiderman.png" alt="Artwork preview 3" />
                        <img src="/ghosty.png" alt="Artwork preview 4" />
                        <img src="/fairy.png" alt="Artwork preview 5" />
                        <img src="/MickeyMiniInParis.png" alt="Artwork preview 6" />
                        <img src="/MinieKüsstMickey.jpg" alt="Artwork preview 7" />
                        <img src="/IMG_0067.PNG" alt="Artwork preview 8" />
                        <img src="/IMG_0068.PNG" alt="Artwork preview 9" />
                        <img src="/IMG_0069.PNG" alt="Artwork preview 10" />
                        <img src="/IMG_0070.PNG" alt="Artwork preview 11" />
                        <img src="/IMG_0071.PNG" alt="Artwork preview 12" />
                    </div>
                    <div class="marquee__track" aria-hidden="true">
                        <img src="/fortnitelama.png" alt="" />
                        <img src="/pikachu.png" alt="" />
                        <img src="/spiderman.png" alt="" />
                        <img src="/ghosty.png" alt="" />
                        <img src="/fairy.png" alt="" />
                        <img src="/MickeyMiniInParis.png" alt="" />
                        <img src="/MinieKüsstMickey.jpg" alt="" />
                        <img src="/IMG_0067.PNG" alt="" />
                        <img src="/IMG_0068.PNG" alt="" />
                        <img src="/IMG_0069.PNG" alt="" />
                        <img src="/IMG_0070.PNG" alt="" />
                        <img src="/IMG_0071.PNG" alt="" />
                    </div>
                </div>
            </section>

            <footer class="footer">
                <div class="socials">
                    <A href="/socials" attr:class="socials__pill">"Alle Socials →"</A>
                </div>
                <small>{format!("© {} SamTV1998 · All rights reserved.", year)}</small>
            </footer>
        </main>
    }
}
