#![allow(unused_imports)]

mod app;
mod components;
mod config;
mod state;

use app::App;

fn main() {
    let renderer = yew::Renderer::<App>::new();
    renderer.render();
    // renderer.hydrate();
}
