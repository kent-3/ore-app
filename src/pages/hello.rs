use dioxus::prelude::*;
use crate::components::Header;

#[component]
pub fn Hello() -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-base-bg text-elements-highEmphasis",
            Header {}
            div {
                class: "flex items-center justify-center min-h-screen pt-20",
                h1 {
                    class: "text-4xl font-bold",
                    "Hello World"
                }
            }
        }
    }
}
