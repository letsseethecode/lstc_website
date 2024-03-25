use crate::components::{EventList, EventView, Home, NotFound};
use yew::prelude::*;
use yew_router::{BrowserRouter, Routable, Switch};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/404")]
    #[not_found]
    NotFound,
    #[at("/event")]
    EventList,
    #[at("/event/:id")]
    EventView { id: String },
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html!(<Home/>),
        Route::NotFound => html!(<NotFound/>),
        Route::EventList => html!(<EventList />),
        Route::EventView { id } => html!(<EventView id={id} />),
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
