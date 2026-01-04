use dioxus::prelude::*;

use crate::{PAGE, functions::navbar_css::Page};

#[component]
pub fn Directory() -> Element {
    use_effect(|| *PAGE.write() = Some(Page::Directory));
    
    rsx! {
        main { class: "min-h-full w-480 max-w-screen p-5",
            h1{class: "text-2xl", "Directory"}
            br {}
            p {class:"text-xl", "Welcome to gruppr.dev"}
        }
    }
}