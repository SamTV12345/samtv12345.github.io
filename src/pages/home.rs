use crate::page_meta::configure_page;
use crate::pages::site::{SiteFooter, CONTACT_EMAIL};
use leptos::prelude::*;
use leptos_router::components::A;

struct Project {
    name: &'static str,
    href: &'static str,
    stack: &'static str,
    text: &'static str,
}

const PROJECTS: &[Project] = &[
    Project {
        name: "PodFetch",
        href: "https://github.com/SamTV12345/PodFetch",
        stack: "Rust, React",
        text: "Selbst gehosteter Podcast-Server: abonnieren, herunterladen und im Browser hören. Mit Helm-Chart und Unraid-Vorlage für den eigenen Server.",
    },
    Project {
        name: "Etherpad Go",
        href: "https://github.com/ether/etherpad-go",
        stack: "Go",
        text: "Neuimplementierung des kollaborativen Editors Etherpad in Go, mit Blick auf Geschwindigkeit, geringen Speicherbedarf und den Betrieb mit mehreren Instanzen.",
    },
    Project {
        name: "SmartOrganizr",
        href: "https://github.com/SamTV12345/SmartOrganizr",
        stack: "Go, React",
        text: "Notenverwaltung für Orchester und Musiker, mit Web-Oberfläche und eigenem Helm-Chart.",
    },
    Project {
        name: "LIVISI SmartHome Frontend",
        href: "https://github.com/SamTV12345/LIVISI-Unofficial-SmartHome-Frontend",
        stack: "TypeScript",
        text: "Inoffizielle Oberfläche für die lokale Smart-Home-Zentrale von LIVISI, ehemals innogy.",
    },
    Project {
        name: "cert-manager-webhook-namecheap",
        href: "https://github.com/SamTV12345/cert-manager-webhook-namecheap",
        stack: "Go, Kubernetes",
        text: "Webhook für cert-manager, um TLS-Zertifikate per DNS-Challenge über Namecheap auszustellen.",
    },
    Project {
        name: "passkey-demo",
        href: "https://github.com/SamTV12345/passkey-demo",
        stack: "WebAuthn",
        text: "Beispielanwendung für die Anmeldung mit Passkeys.",
    },
];

#[component]
pub fn HomePage() -> impl IntoView {
    configure_page("Samuel Schwanzer", "route-site");

    view! {
        <div class="site" lang="de">
            <header class="site-head">
                <div class="site-wrap">
                    <h1 class="site-name">"Samuel Schwanzer"</h1>
                    <p class="site-lede">
                        "Informatiker aus dem Saarland. In meiner Freizeit baue ich Open-Source-Software, vor allem in Rust, Go und TypeScript, und betreibe sie auf meinen eigenen Servern."
                    </p>
                    <ul class="site-contact" role="list">
                        <li><a href="https://github.com/SamTV12345">"GitHub"</a></li>
                        <li><a href="https://www.linkedin.com/in/samuel-schwanzer-733204195/">"LinkedIn"</a></li>
                        <li><a href=format!("mailto:{CONTACT_EMAIL}")>{CONTACT_EMAIL}</a></li>
                    </ul>
                </div>
            </header>

            <main>
                <section class="site-section" aria-labelledby="projekte">
                    <div class="site-wrap">
                        <h2 id="projekte">"Projekte"</h2>
                        <ul class="projects" role="list">
                            {PROJECTS
                                .iter()
                                .map(|p| {
                                    view! {
                                        <li class="project">
                                            <a class="project-link" href=p.href>
                                                <span class="project-name">{p.name}</span>
                                                <span class="project-stack">{p.stack}</span>
                                            </a>
                                            <p class="project-text">{p.text}</p>
                                        </li>
                                    }
                                })
                                .collect_view()}
                        </ul>
                        <p class="site-more">
                            <a href="https://github.com/SamTV12345?tab=repositories">"Alle Repositories auf GitHub"</a>
                        </p>
                    </div>
                </section>

                <section class="site-section feature" aria-labelledby="aktuell">
                    <div class="site-wrap">
                        <h2 id="aktuell">"Aktuell"</h2>
                        <A href="/b250e" attr:class="feature-card">
                            <span class="feature-title">"Mein Mercedes sendet noch."</span>
                            <span class="feature-text">
                                "Mercedes hat die vernetzten Dienste meines Elektroautos abgeschaltet, obwohl das Auto weiter Daten sendet. Die dokumentierte Geschichte eines B\u{a0}250\u{a0}e."
                            </span>
                        </A>
                    </div>
                </section>

                <section class="site-section" aria-labelledby="sonst">
                    <div class="site-wrap">
                        <h2 id="sonst">"Abseits vom Code"</h2>
                        <p class="site-plain">
                            "Ich zeichne gern. Meine Bilder findest du unter "
                            <A href="/art">"Zeichnungen"</A>
                            ", meine Kanäle unter "
                            <A href="/socials">"Social Media"</A>
                            "."
                        </p>
                    </div>
                </section>
            </main>

            <SiteFooter />
        </div>
    }
}
