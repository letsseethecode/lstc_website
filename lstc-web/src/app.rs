use crate::components::{EventListPage, EventViewPage, HomePage, NotFoundPage};
use yew::prelude::*;
use yew_router::{BrowserRouter, Routable, Switch};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
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
pub type Link = yew_router::components::Link<Route>;

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html!(<HomePage/>),
        Route::NotFound => html!(<NotFoundPage/>),
        Route::EventList => html!(<EventListPage />),
        Route::EventView { id } => html!(<EventViewPage id={id} />),
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
