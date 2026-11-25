//! Gemeinsame Bausteine der Startseite sowie Impressum und Datenschutzerklärung.
//!
//! Kontaktdaten stehen nur hier, damit sie an einer Stelle gepflegt werden.

use crate::page_meta::configure_page;
use leptos::prelude::*;
use leptos_router::components::A;

pub const CONTACT_EMAIL: &str = "media@samtv.fyi";
const NAME: &str = "Samuel Schwanzer";
const STREET: &str = "Zur Schafbrücke 5";
const CITY: &str = "66359 Bous";
const STAND: &str = "25. November 2026";

#[component]
pub fn SiteFooter() -> impl IntoView {
    view! {
        <footer class="site-foot">
            <div class="site-wrap site-foot-row">
                <A href="/">{NAME}</A>
                <nav aria-label="Rechtliches">
                    <A href="/impressum">"Impressum"</A>
                    <A href="/datenschutz">"Datenschutz"</A>
                </nav>
            </div>
        </footer>
    }
}

#[component]
pub fn ImpressumPage() -> impl IntoView {
    configure_page("Impressum – Samuel Schwanzer", "route-site");

    view! {
        <div class="site" lang="de">
            <main class="site-section legal">
                <div class="site-wrap">
                    <p class="legal-back"><A href="/">"Zur Startseite"</A></p>
                    <h1>"Impressum"</h1>

                    <h2>"Angaben gemäß § 5 DDG"</h2>
                    <p>{NAME}<br />{STREET}<br />{CITY}<br />"Deutschland"</p>

                    <h2>"Kontakt"</h2>
                    <p>"E-Mail: "<a href=format!("mailto:{CONTACT_EMAIL}")>{CONTACT_EMAIL}</a></p>

                    <h2>"Verantwortlich für den Inhalt nach § 18 Abs. 2 MStV"</h2>
                    <p>{NAME}<br />{STREET}<br />{CITY}</p>

                    <h2>"Haftung für Links"</h2>
                    <p>"Diese Website enthält Links zu externen Websites, etwa zu GitHub und Reddit. Auf deren Inhalte habe ich keinen Einfluss; für sie ist der jeweilige Anbieter verantwortlich. Werden mir Rechtsverletzungen bekannt, entferne ich den betreffenden Link."</p>
                </div>
            </main>
            <SiteFooter />
        </div>
    }
}

#[component]
pub fn DatenschutzPage() -> impl IntoView {
    configure_page("Datenschutz – Samuel Schwanzer", "route-site");

    view! {
        <div class="site" lang="de">
            <main class="site-section legal">
                <div class="site-wrap">
                    <p class="legal-back"><A href="/">"Zur Startseite"</A></p>
                    <h1>"Datenschutzerklärung"</h1>

                    <h2>"Verantwortlicher"</h2>
                    <p>{NAME}<br />{STREET}<br />{CITY}<br />"E-Mail: "<a href=format!("mailto:{CONTACT_EMAIL}")>{CONTACT_EMAIL}</a></p>

                    <h2>"Kurz gesagt"</h2>
                    <p>"Diese Website verwendet keine Cookies, kein Tracking, keine Analysedienste und keine eingebetteten Inhalte Dritter. Schriftarten werden von diesem Server geladen, nicht von Google oder anderen Anbietern."</p>

                    <h2>"Bereitstellung der Website und Server-Logs"</h2>
                    <p>"Beim Aufruf der Website verarbeitet der Server technisch notwendige Daten: IP-Adresse, Datum und Uhrzeit des Abrufs, aufgerufene Adresse, übertragene Datenmenge, Browser und Betriebssystem sowie die zuvor besuchte Seite (Referrer). Das ist nötig, um die Website auszuliefern und ihren sicheren Betrieb zu gewährleisten. Rechtsgrundlage ist mein berechtigtes Interesse nach Art. 6 Abs. 1 lit. f DSGVO."</p>
                    <p>"Gehostet wird die Website "<strong>"[Hoster ergänzen]"</strong>". Die Server-Logs werden nach "<strong>"[Speicherdauer ergänzen]"</strong>" gelöscht."</p>

                    <h2>"Kontakt per E-Mail"</h2>
                    <p>"Wenn du mir eine E-Mail schreibst, verarbeite ich deine Adresse und den Inhalt der Nachricht, um sie zu beantworten (Art. 6 Abs. 1 lit. f DSGVO). Die Nachricht lösche ich, wenn sie erledigt ist und keine gesetzlichen Aufbewahrungspflichten bestehen."</p>

                    <h2>"Links zu anderen Websites"</h2>
                    <p>"Links zu GitHub, LinkedIn, Reddit und anderen Diensten werden erst beim Anklicken aufgerufen. Ab dann gilt die Datenschutzerklärung des jeweiligen Anbieters."</p>

                    <h2>"Deine Rechte"</h2>
                    <p>"Du hast das Recht auf Auskunft (Art. 15 DSGVO), Berichtigung (Art. 16), Löschung (Art. 17), Einschränkung der Verarbeitung (Art. 18), Datenübertragbarkeit (Art. 20) und Widerspruch (Art. 21). Schreib mir dazu einfach eine E-Mail. Außerdem kannst du dich bei einer Datenschutz-Aufsichtsbehörde beschweren, zum Beispiel beim Unabhängigen Datenschutzzentrum Saarland."</p>

                    <p class="legal-stand">"Stand: "{STAND}</p>
                </div>
            </main>
            <SiteFooter />
        </div>
    }
}
