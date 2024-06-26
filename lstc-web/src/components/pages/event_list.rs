use crate::components::{Console, Template};
use crate::state::{State, StateAction, StateClient, StateReducer};
use crate::{
    app::{Link, Route},
    config::Config,
};
use chrono::{Datelike, Local};
use gloo_net::http::Request;
use lstc_apigw::{Envelope, Event};
use yew::{function_component, html, use_context, use_effect_with, use_state, Html};

#[function_component(EventListPage)]
pub fn event_list() -> Html {
    let today = Local::now().date_naive();
    let state = use_context::<StateClient>().expect("State missing");
    {
        let state = state.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                state.load_events(today.year()).await;
            });
        });
    }
    let past = state
        .reducer
        .current_event_list
        .iter()
        .filter(|e| e.date < today);
    let future = state
        .reducer
        .current_event_list
        .iter()
        .filter(|e| e.date >= today);
    html!(
        <Template>
            <h3><Console text="Events" /></h3>
            <h4>{"Future Events"}</h4>
            <table>
                {future.map(|event| html!(
                    <tr>
                        <td width={200}>
                            <Link to={Route::EventView { id: event.date.to_string() }}>
                                {event.date.to_string()}
                            </Link>
                        </td>
                        <td>{event.title.clone()}</td>
                    </tr>
                )).collect::<Html>()}
            </table>
            <h4>{"Past Events"}</h4>
            <table>
                {past.map(|event| html!(
                    <tr>
                        <td width={200}>
                            <Link to={Route::EventView { id: event.date.to_string() }}>
                                {event.date.to_string()}
                            </Link>
                        </td>
                        <td>{event.title.clone()}</td>
                    </tr>
                )).collect::<Html>()}
            </table>
        </Template>
    )
}
