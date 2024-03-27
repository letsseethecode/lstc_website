use yew::{function_component, html, Html, Properties};

use crate::components::Template;

#[derive(Properties, PartialEq)]
pub struct EventViewPageProps {
    pub id: String,
}

#[function_component(EventViewPage)]
pub fn event_view(props: &EventViewPageProps) -> Html {
    html!(
        <Template>
            <h3>{format!("Event View : {}", props.id)}</h3>
            <p>{"
                We're still building this bit.  If you'd like to contribute
                then clone the repository and submit a PR.
            "}</p>
        </Template>
    )
}
