use crate::components::ArtCard;
use crate::page_meta::configure_page;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn ArtPage() -> impl IntoView {
    configure_page("My art library", "route-art");

    view! {
        <main class="art-main">
            <section class="art-shell">
                <header class="art-hero">
                    <h1 class="header-art">"Art Library"</h1>
                    <p class="art-subtitle">
                        "Illustrationen, Skizzen und kleine Experimente aus meinem Feed."
                    </p>
                    <A href="/" attr:class="art-back">
                        "← Zur Startseite"
                    </A>
                </header>

                <section class="arts" aria-label="Artwork list">
                    <ArtCard
                        author="samtv1998"
                        title="Minnie und Mickey in Paris"
                        description="Minnie und Mickey in Paris. Mickey erzählt Minnie einen Witz. Sie schwelgt in ihren Träumen und lacht."
                        href="/MickeyMiniInParis.png"
                        date="04.07.2024"
                    />
                    <ArtCard
                        author="samtv1998"
                        title="Minnie küsst Mickey"
                        description="Minnie und Mickey küssen sich. Mickey hält Minnie in seinen Armen"
                        href="/MinieKüsstMickey.jpg"
                        date="04.07.2024"
                    />
                    <ArtCard
                        date="03.07.2024"
                        author="samtv1998"
                        href="/zaehlerschrank.jpg"
                        title="Zählerschrankliste für Port-Beschriftung"
                        description="Eine hübsche Liste, die man sich in seinen Zählerschrank hängen kann."
                    />
                    <ArtCard
                        author="samtv1998"
                        title="Stillleben einer Landschaft"
                        description="Eingebettet in ein üppiges, grünes Tal, entfaltet sich eine majestätische Landschaft, gekrönt von einer Reihe hoch aufragender Berge, die ihre zerklüfteten Spitzen in den azurblauen Himmel recken"
                        href="/the-island.jpg"
                        date="02.07.2024"
                    />
                    <ArtCard
                        author="samtv1998"
                        title="Rose mit Delfinrizz"
                        description="Eine Rose mit einem Delfinrizz, der sich um sie windet"
                        href="/rose_rizz.jpg"
                        date="30.06.2024"
                    />
                </section>
            </section>
        </main>
    }
}
