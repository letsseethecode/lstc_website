#![allow(unused_imports)]

use simple_ssr::app::{App, AppProps};

fn main() {
    let api_base_url = std::env!("API_BASE_URL").to_string();
    let renderer = yew::Renderer::<App>::with_props(AppProps { api_base_url });
    renderer.render();
    // renderer.hydrate();
}
