//! Fallseite zum Mercedes-Benz B 250 e (/b250e).
//!
//! Die Seite dokumentiert die Abschaltung der vernetzten Dienste. Bewusst
//! nicht enthalten: FIN, Kennzeichen, Adresse, Kontaktdaten, Namen von
//! Mitarbeitern und Daten des Vorbesitzers.

use crate::page_meta::configure_page;
use leptos_router::components::A;
use leptos::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};

/// Tag, an dem Mercedes die Remote-Funktionen abgeschaltet hat (18.07.2026).
const SUNDOWN_MS: f64 = 1_784_325_600_000.0;
const DAY_MS: f64 = 86_400_000.0;

fn days_since_sundown() -> u32 {
    ((js_sys::Date::now() - SUNDOWN_MS) / DAY_MS).floor().max(0.0) as u32
}

fn prefers_reduced_motion() -> bool {
    window()
        .match_media("(prefers-reduced-motion: reduce)")
        .ok()
        .flatten()
        .map(|m| m.matches())
        .unwrap_or(false)
}

/// Zahl mit Punkt als Tausendertrennzeichen, z. B. 144.838.
fn format_de(n: u32) -> String {
    let digits = n.to_string();
    let mut out = String::new();
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i) % 3 == 0 {
            out.push('.');
        }
        out.push(c);
    }
    out
}

/// Zählt `signal` in `ms` Millisekunden von 0 auf `target` hoch.
fn count_up(signal: WriteSignal<u32>, target: u32, ms: f64) {
    if prefers_reduced_motion() {
        signal.set(target);
        return;
    }
    let start = js_sys::Date::now();
    let handle: Rc<RefCell<Option<IntervalHandle>>> = Rc::new(RefCell::new(None));
    let handle_in = handle.clone();
    let h = set_interval_with_handle(
        move || {
            let p = ((js_sys::Date::now() - start) / ms).min(1.0);
            let eased = 1.0 - (1.0 - p).powi(3);
            signal.set((target as f64 * eased).round() as u32);
            if p >= 1.0 {
                if let Some(h) = handle_in.borrow_mut().take() {
                    h.clear();
                }
            }
        },
        Duration::from_millis(16),
    );
    if let Ok(h) = h {
        *handle.borrow_mut() = Some(h);
    }
}

/// Blendet alle `.reveal`-Elemente unterhalb von `root` ein, sobald sie ins
/// Bild scrollen. Wird erst aufgerufen, wenn die Seite im DOM hängt, damit es
/// auch bei Navigation innerhalb der App (ohne Neuladen) funktioniert.
fn observe_reveals(root: &web_sys::Element, on_counter: impl Fn() + 'static) {
    let Ok(nodes) = root.query_selector_all(".reveal") else {
        return;
    };

    let callback = Closure::<dyn FnMut(js_sys::Array, IntersectionObserver)>::new(
        move |entries: js_sys::Array, observer: IntersectionObserver| {
            for entry in entries.iter() {
                let entry: IntersectionObserverEntry = entry.unchecked_into();
                if !entry.is_intersecting() {
                    continue;
                }
                let target = entry.target();
                let _ = target.class_list().add_1("in");
                if target.class_list().contains("counter") {
                    on_counter();
                }
                observer.unobserve(&target);
            }
        },
    );

    let options = IntersectionObserverInit::new();
    options.set_threshold(&0.18.into());
    options.set_root_margin("0px 0px -8% 0px");

    let Ok(observer) =
        IntersectionObserver::new_with_options(callback.as_ref().unchecked_ref(), &options)
    else {
        // Ohne IntersectionObserver einfach alles sichtbar machen.
        for i in 0..nodes.length() {
            if let Some(el) = nodes.item(i).and_then(|n| n.dyn_into::<web_sys::Element>().ok()) {
                let _ = el.class_list().add_1("in");
            }
        }
        return;
    };
    callback.forget();

    for i in 0..nodes.length() {
        if let Some(el) = nodes.item(i).and_then(|n| n.dyn_into::<web_sys::Element>().ok()) {
            observer.observe(&el);
        }
    }
}

#[component]
pub fn B250ePage() -> impl IntoView {
    configure_page("Mein Mercedes sendet noch – der Fall B 250 e", "route-b250e");

    let days_total = days_since_sundown();
    let (days, set_days) = signal(0u32);
    let (soc, set_soc) = signal(0u32);
    let (km, set_km) = signal(0u32);

    let page_ref = NodeRef::<leptos::html::Div>::new();
    let started = StoredValue::new(false);

    // Läuft, sobald das Wurzelelement wirklich eingehängt ist, auch nach einem
    // Klick auf einen internen Link.
    Effect::new(move |_| {
        let Some(root) = page_ref.get() else {
            return;
        };
        if started.get_value() {
            return;
        }
        started.set_value(true);
        window().scroll_to_with_x_and_y(0.0, 0.0);
        count_up(set_soc, 84, 1600.0);
        count_up(set_km, 144_838, 1600.0);
        observe_reveals(&root, move || count_up(set_days, days_total, 1400.0));
    });

    view! {
        <div class="cs-page js" lang="de" node_ref=page_ref>
            <header class="cs-hero">
                <div class="cs-hero-pin">
                <div class="wrap cs-hero-grid">
                    <div>
                        <h1>
                            <span class="line">"Mein"</span>" "
                            <span class="line">"Mercedes"</span>" "
                            <span class="line">"sendet noch."</span>
                            <span class="quiet">"Mercedes hört nicht mehr zu."</span>
                        </h1>
                        <p class="lede reveal" style="--i:6">
                            "Ein gepflegtes Elektroauto, das erste in Serie gebaute von Mercedes-Benz. Seit Juli 2026 sind seine vernetzten Dienste abgeschaltet, und einen zeitgemäßen Akku gibt es nicht. Hier ist dokumentiert, was passiert ist."
                        </p>
                    </div>

                    <aside class="cluster reveal from-right" style="--i:4" aria-label="Letzte Datenübertragung des Fahrzeugs an Mercedes">
                        <div class="cluster-head">
                            <span>"Letzte Übertragung an Mercedes"</span>
                            <span class="signal" aria-hidden="true"><span></span><span></span><span></span><span></span></span>
                        </div>
                        <dl class="readout">
                            <div><dt>"Ladestand"</dt><dd>{move || soc.get()}<small>"%"</small></dd></div>
                            <div><dt>"Kilometerstand"</dt><dd>{move || format_de(km.get())}<small>"km"</small></dd></div>
                            <div><dt>"Reifendruck vorne"</dt><dd>"2,55"<small>"/"</small>"2,63"<small>"bar"</small></dd></div>
                            <div><dt>"Reichweite"</dt><dd>"117"<small>"km"</small></dd></div>
                        </dl>
                        <p class="stamp">
                            <strong>"10. Oktober 2026, 13:53 Uhr"</strong>
                            " laut Datenauskunft von Mercedes nach Art. 15 DSGVO"
                        </p>
                    </aside>
                </div>

                <div class="wrap counter reveal" style="--i:8">
                    <output>{move || days.get()}</output>
                    <span>"Tage ohne vernetzte Dienste, seit Mercedes die Remote-Funktionen abgeschaltet hat"</span>
                </div>
                </div>
            </header>

            <main>
                <section>
                    <div class="wrap measure">
                        <h2 class="reveal">"Sechs Antworten, keine Ursache"</h2>
                        <p class="intro reveal" style="--i:1">"Warum die App keine Fahrzeugdaten mehr zeigt, hat mir Mercedes über Wochen immer wieder anders erklärt."</p>
                    </div>
                    <div class="wrap">
                        <ol class="excuses">
                            <li class="reveal"><span class="when">"Sommer 2026, Werkstatt"</span><s class="strike"><span>"Das Auto ist zu alt."</span></s></li>
                            <li class="reveal" style="--i:1"><span class="when">"08.09.2026"</span><s class="strike"><span>"Das Digital Extra ist nicht mehr kompatibel."</span></s></li>
                            <li class="reveal" style="--i:2">
                                <span class="when">"14.09.2026, Telefon"</span>
                                <div>
                                    <s class="strike"><span>"Das 3G-Netz ist abgeschaltet."</span></s>
                                    <p class="note">"Einen Tag später schreibt Mercedes selbst: Das Auto funkt über 2G, und das 2G-Netz ist in Deutschland weiterhin verfügbar."</p>
                                </div>
                            </li>
                            <li class="reveal" style="--i:3"><span class="when">"17.09.2026, schriftlich"</span><s class="strike"><span>"Die Ursache ist nicht abschließend geklärt."</span></s></li>
                            <li class="reveal last" style="--i:4"><span class="when">"6. Oktober 2026"</span><q>"Derzeit nicht bekannt."</q></li>
                            <li class="reveal press" style="--i:5">
                                <span class="when">"November 2026, gegenüber der Presse"</span>
                                <div>
                                    <s class="strike"><span>"Die Netzbetreiber unterstützen kein 2G mehr."</span></s>
                                    <p class="note">"So begründete Mercedes den Ausfall auf die Anfrage eines Journalisten. Mir hatte Mercedes am 15. September schriftlich das Gegenteil mitgeteilt: Das 2G-Netz sei in Deutschland weiterhin verfügbar. Die Netzbetreiber schalten 2G erst 2028 ab, und mein Auto hat noch im Oktober über dieses Netz Daten an Mercedes gesendet."</p>
                                </div>
                            </li>
                        </ol>
                        <p class="after-excuses measure reveal">
                            "Zwischen der fünften und der sechsten Antwort, am 2. November 2026, hat Mercedes den Fall für „abschließend geprüft“ erklärt."
                        </p>
                    </div>
                </section>

                <section>
                    <div class="wrap measure">
                        <h2 class="reveal">"Was Mercedes' eigene Daten zeigen"</h2>
                        <p class="intro reveal" style="--i:1">"Auf meinen Antrag nach Art. 15 DSGVO hat Mercedes mir die gespeicherten Daten zu mir und meinem Auto herausgegeben, wenn auch unvollständig. Schon darin steht, was mir am Telefon niemand gesagt hat."</p>
                        <ul class="facts">
                            <li class="reveal"><strong>"Das Auto sendet weiter."</strong>" Ladestand, Kilometerstand, Reichweite und Reifendruck kamen noch am 20., 22. und 23. September und zuletzt am 10. Oktober 2026 bei Mercedes an. Das Kommunikationsmodul funktioniert."</li>
                            <li class="reveal" style="--i:1"><strong>"Die Abschaltung war ein Vorgang."</strong>" Am 18. Juli 2026 vermerkt Mercedes „Sundown of Remote Extras“. Ab dem 7. August kommen auch keine Statusmeldungen mehr in der App an."</li>
                            <li class="reveal" style="--i:2"><strong>"Mercedes kennt das Muster."</strong>" In den eigenen Fallnotizen steht nach den Werkstattbesuchen: Funktionen „zunächst wieder verfügbar, nach ca. zwei Tagen jedoch erneut ausgefallen“."</li>
                            <li class="reveal" style="--i:3"><strong>"Ein Dienst hängt fest."</strong>" Einer der Dienste meines Autos steht auf „ACTIVATION_PENDING“, obwohl als Sollzustand „ACTIVE“ eingetragen ist."</li>
                        </ul>
                    </div>
                </section>

                <section>
                    <div class="wrap measure">
                        <h2 class="reveal">"Was passiert ist"</h2>
                        <p>"Alle Einträge sind durch Schriftverkehr, Werkstattrechnungen oder die Datenauskunft von Mercedes belegt."</p>
                    </div>
                    <div class="wrap measure">
                        <ol class="timeline">
                            <li class="reveal" style="--i:1"><time datetime="2025-12">"Dez. 2025"</time><p>"Ich kaufe den B\u{a0}250\u{a0}e, Erstzulassung 12/2014. Das Auto wird meinem Mercedes-me-Konto zugeordnet."</p></li>
                            <li class="reveal" style="--i:2"><time datetime="2026-01">"Jan. 2026"</time><p>"Service bei einer Mercedes-Vertragswerkstatt. Dabei wird ein Fehler des Notrufsystems behoben, indem das Kommunikationsmodul neu programmiert wird. Danach funktioniert der Notruf wieder."</p></li>
                            <li class="key reveal" style="--i:0"><time datetime="2026-07-18">"18.07.2026"</time><p>"Mercedes stellt die Remote-Funktionen ein, darunter die Vorklimatisierung per App. Intern heißt der Vorgang "<strong>"„Sundown of Remote Extras“"</strong>"."</p></li>
                            <li class="reveal" style="--i:1"><time datetime="2026-08-07">"07.08.2026"</time><p>"Auch die Anzeige von Ladestand, Reichweite und Reifendruck in der App aktualisiert sich nicht mehr."</p></li>
                            <li class="reveal" style="--i:0"><time datetime="2026-08-21">"21.08.2026"</time><p>"Die Werkstatt lehnt einen Ersatzwagen ab."</p></li>
                            <li class="reveal" style="--i:2"><time datetime="2026-09-01">"01.09.2026"</time><p>"Erste Neucodierung des Kommunikationsmoduls in der Vertragswerkstatt, für 157,68 €. Die Funktionen laufen etwa zwei Tage. Die Kosten erstattet die Werkstatt im November aus Kulanz."</p></li>
                            <li class="reveal" style="--i:0"><time datetime="2026-09-08">"08.09.2026"</time><p>"Zweite Neucodierung, kostenlos, gleiches Ergebnis."</p></li>
                            <li class="reveal" style="--i:1"><time datetime="2026-09-14">"14.09.2026"</time><p>"Am Telefon vergleicht die Kundenbetreuung mein Auto mit einem iPhone: Nach zehn Jahren könne man nicht mehr alle Funktionen erwarten."</p></li>
                            <li class="reveal" style="--i:2"><time datetime="2026-09-15">"15.09.2026"</time><p>"Mercedes lehnt eine kostenlose Diagnose, einen Ersatzwagen und eine Kostenübernahme ab."</p></li>
                            <li class="reveal" style="--i:2"><time datetime="2026-09-16">"16.09.2026"</time><p>"Brief an den Vorstandsvorsitzenden."</p></li>
                            <li class="key reveal" style="--i:0"><time datetime="2026-10-06">"06.10.2026"</time><p>"Mercedes schreibt, die Ursache sei „derzeit nicht bekannt“, und bestätigt den iPhone-Vergleich schriftlich."</p></li>
                            <li class="key reveal" style="--i:1"><time datetime="2026-10-10">"10.10.2026"</time><p>"Laut Datenauskunft die letzte Übertragung des Fahrzeugs an Mercedes: Ladestand, Kilometerstand, Reifendruck."</p></li>
                            <li class="reveal" style="--i:2"><time datetime="2026-11-02">"02.11.2026"</time><p>"Mercedes erklärt den Fall für abgeschlossen."</p></li>
                            <li class="reveal" style="--i:0"><time datetime="2026-11-11">"11.11.2026"</time><p>"Antrag bei der Universalschlichtungsstelle des Bundes. Mercedes nimmt an Verbraucherschlichtungen grundsätzlich nicht teil."</p></li>
                            <li class="key reveal" style="--i:1"><time datetime="2026-11-19">"19.11.2026"</time><p>"Förmliche Aufforderung an Mercedes mit Frist bis zum 21. Dezember 2026."</p></li>
                        </ol>
                    </div>
                </section>

                <section>
                    <div class="wrap measure">
                        <h2 class="reveal">"Spätestens 2028 ist ohnehin Schluss"</h2>
                        <p class="reveal" style="--i:1">"Selbst wenn Mercedes die Dienste heute wieder einschaltet: Das Modul in meinem Auto funkt nur über 2G, und die deutschen Netzbetreiber schalten 2G im Jahr 2028 ab. Die Telekom zum 30. Juni, Vodafone im September, O2 in der zweiten Jahreshälfte. Einen Nachfolger für das Modul gibt es laut Mercedes nicht, ein Umstieg auf 4G oder 5G sei nicht möglich."</p>
                        <p class="reveal" style="--i:2">"Damit hat jedes vernetzte Auto dieser Generation ein eingebautes Verfallsdatum, das nicht der Käufer bestimmt, sondern der Hersteller und die Netzbetreiber."</p>
                    </div>
                </section>

                <section>
                    <div class="wrap">
                        <figure class="pull reveal zoom">
                            <blockquote>"Rund 80 Kilometer im Winter. An kalten Tagen eher 60."</blockquote>
                            <figcaption>"Der B\u{a0}250\u{a0}e heizt mit einem elektrischen Zuheizer mit bis zu 6 kW. Ohne Vorklimatisierung am Kabel kommt diese Energie aus dem Akku. Einen Ersatz- oder Upgrade-Akku mit aktueller Zelltechnik bietet Mercedes nicht an, auch nicht gegen Bezahlung."</figcaption>
                        </figure>
                    </div>
                </section>

                <section>
                    <div class="wrap measure">
                        <h2 class="reveal">"Was ich von Mercedes erwarte"</h2>
                        <ol class="demands">
                            <li class="reveal" style="--i:1"><div><h3>"Die Dienste wiederherstellen"</h3><p>"Oder eine Nachrüstlösung anbieten, etwa ein Kommunikationsmodul für aktuelle Mobilfunknetze."</p></div></li>
                            <li class="reveal" style="--i:2"><div><h3>"Einen zeitgemäßen Akku anbieten"</h3><p>"Mit mehr nutzbarer Kapazität und zu einem fairen Preis, den ich selbst trage."</p></div></li>
                            <li class="reveal" style="--i:3"><div><h3>"Sagen, wie lange es Ersatzteile gibt"</h3><p>"Für Hochvoltakkus und Kommunikationsmodule des B\u{a0}250\u{a0}e."</p></div></li>
                            <li class="reveal" style="--i:4"><div><h3>"Den Zugang zu meinen Fahrzeugdaten öffnen"</h3><p>"So, wie es der Data Act vorsieht, auch für Anbieter meiner Wahl."</p></div></li>
                            <li class="reveal" style="--i:5"><div><h3>"Freien Werkstätten die Technik zugänglich machen"</h3><p>"Damit andere reparieren und nachrüsten können, wenn Mercedes es nicht tut."</p></div></li>
                        </ol>
                    </div>
                </section>

                <section>
                    <div class="wrap measure">
                        <h2 class="reveal">"Wo der Fall gerade liegt"</h2>
                        <table class="status reveal" style="--i:1">
                            <thead><tr><th scope="col">"Stelle"</th><th scope="col">"Stand"</th></tr></thead>
                            <tbody>
                                <tr><td>"Mercedes-Benz AG"</td><td><span class="dot open" aria-hidden="true"></span>"Frist bis 21.12.2026 läuft"</td></tr>
                                <tr><td>"Universalschlichtungsstelle des Bundes"</td><td><span class="dot open" aria-hidden="true"></span>"Antrag eingereicht"</td></tr>
                                <tr><td>"Datenschutzaufsicht Baden-Württemberg"</td><td><span class="dot open" aria-hidden="true"></span>"Beschwerde wegen unvollständiger Auskunft"</td></tr>
                                <tr><td>"Europäische Kommission"</td><td><span class="dot open" aria-hidden="true"></span>"Eingabe zu digitaler Obsoleszenz und Data Act"</td></tr>
                                <tr><td>"Petitionsausschuss des Bundestages"</td><td><span class="dot open" aria-hidden="true"></span>"Petition für Mindest-Supportzeiten"</td></tr>
                                <tr><td>"Runder Tisch Reparatur"</td><td><span class="dot done" aria-hidden="true"></span>"Als Praxisbeispiel aufgenommen"</td></tr>
                            </tbody>
                        </table>
                    </div>
                </section>

                <section class="owners">
                    <div class="wrap measure">
                        <h2 class="reveal">"Fährst du auch einen B\u{a0}250\u{a0}e?"</h2>
                        <p class="intro reveal" style="--i:1">"Dann bist du wahrscheinlich nicht allein mit leerer App und knapper Winterreichweite. Je mehr Halter sich melden, desto schwerer lässt sich das als Einzelfall abtun."</p>
                        <div class="links reveal" style="--i:2">
                            <a class="cs-btn" href="https://www.reddit.com/r/luftablassen/s/223BAJmhLq">"Zur Diskussion auf Reddit"</a>
                            <a class="cs-btn cs-ghost" href="https://www.reddit.com/r/luftablassen/s/SnwGna7UyZ">"Das neueste Update lesen"</a>
                        </div>
                    </div>
                </section>
            </main>

            <footer>
                <div class="wrap">
                    <p>"Stand: 25. November 2026. Diese Seite gibt meine persönliche Erfahrung wieder. Alle Tatsachen sind durch Schriftverkehr, Rechnungen und die Datenauskunft von Mercedes nach Art. 15 DSGVO belegt. Namen von Mitarbeitern nenne ich bewusst nicht."</p>
                    <p class="cs-legal">
                        <A href="/">"Startseite"</A>
                        <A href="/impressum">"Impressum"</A>
                        <A href="/datenschutz">"Datenschutz"</A>
                    </p>
                </div>
            </footer>
        </div>
    }
}
