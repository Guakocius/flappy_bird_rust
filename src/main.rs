use dioxus::prelude::*;

use flappy_bird_rust::{components::navbar::Route, enums::signals::ThemeSignal};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut is_light_mode = use_signal(|| false);
    use_context_provider(|| ThemeSignal(is_light_mode));
    let theme_class = if is_light_mode() { "light-mode" } else { "" };

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div {
            class: "{theme_class}", id: "app-root",
            Router::<Route> {}

        }
    }
}
