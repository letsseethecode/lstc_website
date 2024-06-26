use chrono::{Datelike, NaiveDate};
use gloo_net::http::Request;
use lstc_domain::{Envelope, Event};
use yew::{Reducible, UseReducerHandle};

#[derive(Debug, Clone, PartialEq)]
pub struct State {
    // The single event loaded
    pub current_event: Option<Event>,
    // The list of events loaded
    pub current_event_list_year: i32,
    pub current_event_list: Vec<Event>,
}

pub enum StateAction {
    LoadEvent(Event),
    LoadEvents(i32, Vec<Event>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct StateClient {
    pub base_url: String,
    pub reducer: UseReducerHandle<State>,
}

impl StateClient {
    pub fn new(base_url: String, reducer: UseReducerHandle<State>) -> Self {
        Self { base_url, reducer }
    }
}

impl Reducible for State {
    type Action = StateAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            StateAction::LoadEvent(event) => Self {
                current_event: Some(event.clone()),
                current_event_list_year: self.current_event_list_year,
                current_event_list: self.current_event_list.clone(),
            }
            .into(),
            StateAction::LoadEvents(year, events) => Self {
                current_event_list_year: year,
                current_event_list: events,
                current_event: self.current_event.clone(),
            }
            .into(),
        }
    }
}

impl State {
    pub fn new() -> Self {
        State {
            current_event: None,
            current_event_list_year: 0,
            current_event_list: vec![],
        }
    }
}

impl StateClient {
    pub async fn load_events(&self, year: i32) {
        if self.reducer.current_event_list_year != year {
            let url = format!("{}/event/{}", self.base_url, year);
            let response = Request::get(url.as_str()).send().await.unwrap();
            let result: Envelope<Vec<Event>> = response.json().await.unwrap();
            let mut events = result.data.clone();
            events.sort_by(|a, b| b.date.cmp(&a.date));
            self.reducer.dispatch(StateAction::LoadEvents(year, events));
        }
    }

    pub async fn load_event(&self, date: &NaiveDate) {
        if !self
            .reducer
            .current_event
            .clone()
            .map_or(false, |e| e.date == *date)
        {
            let year = date.year();
            let month = date.month();
            let day = date.day0() + 1;
            let url = format!("{}/event/{}/{}/{}", self.base_url, year, month, day,);
            let response = Request::get(url.as_str()).send().await.unwrap();
            let result: Envelope<Event> = response.json().await.unwrap();
            self.reducer.dispatch(StateAction::LoadEvent(result.data));
        }
    }
}
