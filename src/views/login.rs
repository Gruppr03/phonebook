use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{functions::navbar_css::Page, PAGE};

#[derive(Serialize, Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

#[component]
pub fn Login() -> Element {
    use_effect(|| *PAGE.write() = Some(Page::Login));

    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut navigator = use_navigator();

    rsx! {
        main { class: "min-h-full w-480 max-w-screen p-5",
            h1{class: "text-2xl", "Login"}
            br {}
            p {class:"text-xl", "Welcome to gruppr.dev"}

            div { class: "justify-center px-6 py-12 lg:px-8",
                div { class: "sm:mx-auto sm:w-full sm:max-w-sm",
                    h2 {class: "mt-10 text-center text-2xl/9 font-bold tracking-tight text-black", "Sign in to your account" }
                }


            }
        }
    }
}
