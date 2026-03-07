mod app;
mod components;
mod page_meta;
mod pages;

use app::App;
use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}
