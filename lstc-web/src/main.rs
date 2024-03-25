#![allow(unused_imports)]

mod app;
mod components;

use app::App;

fn main() {
    let renderer = yew::Renderer::<App>::new();
    renderer.render();
    // renderer.hydrate();
}
