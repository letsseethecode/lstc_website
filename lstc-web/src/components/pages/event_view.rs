use yew::{function_component, html, Html, Properties};

use crate::components::Template;

#[derive(Properties, PartialEq)]
pub struct EventViewPageProps {
    pub id: String,
}

#[function_component(EventViewPage)]
pub fn event_view(props: &EventViewPageProps) -> Html {
    html!(
        <Template>{format!("Event View : {}", props.id)}</Template>
    )
}
