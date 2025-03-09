use std::fs;
use dioxus::prelude::*;

use components::Navbar;
use views::{ Home};
use crate::rssPub::rss_fetcher::get_content;
use crate::rssPub::scrapper::get_all_content;
use crate::rssPub::db_loader::*;
use crate::rssPub::loader::load_data;

mod components;
mod views;
mod rssPub;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {}}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

// fn main() {
//     dioxus::launch(App);
//      init_data();
// }

fn main() {
    load_data();
}
#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Route> {}
    }
}
