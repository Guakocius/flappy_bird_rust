use dioxus::prelude::*;

use crate::enums::signals::ThemeSignal;

const NAVBAR_CSS: Asset = asset!("/assets/navbar.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {}
}

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div {
            id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p { "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }

            // Navigation links
            Link {
                to: Route::Blog { id: id - 1 },
                "Previous"
            }
            span { " <---> " }
            Link {
                to: Route::Blog { id: id + 1 },
                "Next"
            }
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    let mut theme = use_context::<ThemeSignal>().0;

    rsx! {
    document::Link { rel: "stylesheet", href: NAVBAR_CSS }
    nav {
        id: "navbar",
        for f in ["Index", "Register", "Login", "Game", "Leaderboard"] {
            let route = match f {
                "Index" => Route::Home {},
                "Register" => Route::Register {},
                "Login" => Route::Login {},
                "Game" => Route::Game {},
                "Leaderboard" => Route::Leaderboard {},
                _ => Route::Home {}
            };
            Link {
                to: route,
                "{f}"
            }
    }
        button {
            id: "darkmode",
            onclick: move |_| theme.toggle(),
            if theme() { "Darkmode" } else { "Lightmode" }
        }
        }
    }
}
