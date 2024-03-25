use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct EventViewProps {
    pub id: String,
}

#[function_component(EventView)]
pub fn event_view(props: &EventViewProps) -> Html {
    html!(
        <div>{format!("Event View : {}", props.id)}</div>
    )
}
