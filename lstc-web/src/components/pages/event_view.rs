use chrono::{Date, Datelike, NaiveDate};
use lstc_apigw::Event;
use yew::{function_component, html, use_context, use_effect_with, use_state, Html, Properties};

use crate::{
    components::{Console, Template},
    state::{State, StateClient},
};

#[derive(Properties, PartialEq)]
pub struct EventViewPageProps {
    pub id: String,
}

#[function_component(EventViewPage)]
pub fn event_view(props: &EventViewPageProps) -> Html {
    let date = NaiveDate::parse_from_str(props.id.as_str(), "%Y-%m-%d").expect("date");
    let state = use_context::<StateClient>().expect("State missing");
    {
        let state = state.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                state.load_event(&date).await;
            });
        });
    }

    let event = state.reducer.current_event.clone();

    html!(
        <Template>
            {if let Some(e) = event {
                html!(
                    <>
                        <h3><Console text={e.date.clone().to_string()} /></h3>
                        <h4>{e.headline.clone()}</h4>
                        <pre>{e.body.clone()}</pre>
                    </>
                )
            } else {
                html!(
                    <>
                        <h3>{"Loading..."}</h3>
                    </>
                )
            }}
        </Template>
    )
}
