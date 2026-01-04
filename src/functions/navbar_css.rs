use crate::PAGE;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Page {
    Home,
    Directory,
    Favourites,
    Login,
}

pub fn page_select_css<'a>(page: Page) -> &'a str {
    if PAGE == Some(page) {
        "bg-blue-400 overflow-hidden min-w-fit p-1.5 m-1.5 rounded-md text-white"
    } else {
        "hover:bg-blue-300 hover:text-zinc-800 overflow-hidden min-w-fit p-1.5 m-1.5 rounded-md"
    }
}