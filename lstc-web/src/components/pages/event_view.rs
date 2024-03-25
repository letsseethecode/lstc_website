use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct EventViewPageProps {
    pub id: String,
}

#[function_component(EventViewPage)]
pub fn event_view(props: &EventViewPageProps) -> Html {
    html!(
        <div>{format!("Event View : {}", props.id)}</div>
    )
}
