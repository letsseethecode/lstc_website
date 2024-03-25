mod app;

use app::App;

fn main() {
    let renderer = yew::Renderer::<App>::new();
    renderer.render();
    // renderer.hydrate();
}
