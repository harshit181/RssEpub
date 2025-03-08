use crate::components::{Echo, Hero};
use dioxus::prelude::*;
use crate::views::login::Login;

#[component]
pub fn Home() -> Element {
    rsx! {
        Login {}
    }
}
