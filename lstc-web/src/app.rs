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

#[derive(Properties, PartialEq)]
pub struct AppProps {
    pub api_base_url: String,
}

#[function_component(App)]
pub fn app(props: &AppProps) -> Html {
    let config = use_state(Config::new);
    let state = use_reducer(State::new);
    let util = StateClient::new(props.api_base_url.clone(), state);
    html! {
        <ContextProvider<Config> context={(*config).clone()}>
            <ContextProvider<StateClient> context={util} >
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </ContextProvider<StateClient>>
        </ContextProvider<Config>>
    }
}
