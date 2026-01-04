use dioxus::prelude::*;

use crate::PAGE;

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    use_effect(|| *PAGE.write() = None);
    
    rsx! {
        main { class: "p-10",
            h1{class: "text-2xl", "Page not found"}
            br {}
            p {class:"text-xl", "You tried to navigate to {route:?}"}
        }
    }
}