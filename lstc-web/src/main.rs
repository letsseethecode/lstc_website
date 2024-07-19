#![allow(unused_imports)]

mod app;
mod components;
mod config;
mod state;

use app::{App, AppProps};

fn main() {
    let api_base_url = std::env!("API_BASE_URL").to_string();
    let renderer = yew::Renderer::<App>::with_props(AppProps { api_base_url });
    renderer.render();
    // renderer.hydrate();
}
