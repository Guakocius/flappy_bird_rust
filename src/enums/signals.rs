use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub struct ThemeSignal(pub Signal<bool>);
