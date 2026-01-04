use dioxus::prelude::*;
use crate::views::{navbar::Navbar, pagenotfound::PageNotFound, home::Home, favourites::Favourites, directory::Directory, login::Login};
use crate::functions::navbar_css::Page;

mod views;
mod functions;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/output.css");

// npx @tailwindcss/cli -i ./input.css -o ./assets/output.css --watch

static PAGE: GlobalSignal<Option<Page>> = Signal::global(|| None);

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/directory")]
    Directory {},
    #[route("/favourites")]
    Favourites {},
    #[route("/login")]
    Login {},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound {route: Vec<String>}
}

fn main() {
    dioxus::launch(App);
}

// app page
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        meta {name: "viewport", content: "width=device-width, initial-scale=1.0"}
        Router::<Route> {}
    }
}
