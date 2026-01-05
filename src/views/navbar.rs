use dioxus::prelude::*;

use crate::{
    functions::navbar_css::{page_select_css, Page},
    Route,
};

const LOGO_LARGE_SVG: Asset = asset!("/assets/SVGs/logo_large.svg");
const LOGO_SMALL_SVG: Asset = asset!("/assets/SVGs/logo_small.svg");
const HOME_SVG: Asset = asset!("/assets/SVGs/home.svg");
const DIRECTORY_SVG: Asset = asset!("/assets/SVGs/directory.svg");
const FAVOURITES_SVG: Asset = asset!("/assets/SVGs/favourites.svg");
const LOGOUT_SVG: Asset = asset!("/assets/SVGs/logout.svg");

#[component]
pub fn Navbar() -> Element {
    rsx! {
        main {class: "flex relative grid grid-cols-8 bg-zinc-800",
            body { class: "flex flex-col min-h-screen content-center col-span-8 mx-auto z-90",
                nav { class: "sticky top-0 bg-zinc-200 z-99",
                    div { class: "flex content-normal text-nowrap border-b border-zinc-400",
                        div { class: "flex flex-1 text-center min-w-fit md:min-w-1/2 content-center items-center",
                            div {class: "p-1.5 m-1.5",
                                img {class: "hidden md:inline w-30", src: LOGO_LARGE_SVG}, img {class: "inline md:hidden w-5", src: LOGO_SMALL_SVG}
                            }
                            Link { class: page_select_css(Page::Home),
                                to: Route::Home {}, a {class:"hidden md:inline", "Home"} img {class: "inline md:hidden", src: HOME_SVG}
                            },
                            Link { class: page_select_css(Page::Directory),
                                to: Route::Directory {}, a {class:"hidden md:inline", "Directory"} img {class: "inline md:hidden", src: DIRECTORY_SVG}
                            },
                            Link { class: page_select_css(Page::Favourites),
                                to: Route::Favourites {}, a {class:"hidden md:inline", "Favourites"} img {class: "inline md:hidden", src: FAVOURITES_SVG}
                            },
                        }

                        div { class: "flex flex-1 grid grid-cols-4 text-center min-w-fit md:min-w-1/2 content-center",
                            div {class: "col-span-3"}
                            Link { class: page_select_css(Page::Login),
                                to: Route::Login {}, a {class:"hidden md:inline", "Login"} img {class: "inline md:hidden", src: LOGOUT_SVG}
                            },
                        }
                    }
                }

                div { class: "flex-1 bg-white",
                    Outlet::<Route> {}
                }
            }
        }
    }
}
