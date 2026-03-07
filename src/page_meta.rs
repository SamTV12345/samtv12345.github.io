use leptos::prelude::*;

pub fn configure_page(title: &'static str, body_class: &'static str) {
    Effect::new(move |_| {
        document().set_title(title);
        if let Some(body) = document().body() {
            body.set_class_name(body_class);
        }
    });
}
