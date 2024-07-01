use crate::{
    components::{EventListPage, EventViewPage, HomePage, NotFoundPage},
    config::Config,
    state::{State, StateClient},
};
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
    let config = use_state(Config::new);
    let state = use_reducer(State::new);
    let base_url = "https://0d99uja6bd.execute-api.eu-west-2.amazonaws.com/production".to_string();
    let util = StateClient::new(base_url, state);
    html! {
        <ContextProvider<Config> context={(*config).clone()}>
            <ContextProvider<StateClient> context={util}>
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </ContextProvider<StateClient>>
        </ContextProvider<Config>>
    }
}
