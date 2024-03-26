use yew::{function_component, html, Html};

use crate::components::Template;

#[function_component(EventListPage)]
pub fn event_list() -> Html {
    html!(
        <Template>
            {"Event List"}
        </Template>
    )
}
