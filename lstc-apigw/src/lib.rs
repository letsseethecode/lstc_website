use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Warning {
    pub message: String,
}

impl Warning {
    pub fn new<S: ToString>(message: S) -> Self {
        Warning {
            message: message.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Fault {
    pub message: String,
}

impl Fault {
    pub fn new<S: ToString>(message: S) -> Self {
        Fault {
            message: message.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Envelope<D> {
    pub message: String,
    pub warnings: Vec<Warning>,
    pub faults: Vec<Fault>,
    pub data: D,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Event {
    pub date: NaiveDate,
    pub headline: String,
    pub body: String,
}

impl Event {
    pub fn new<S: ToString>(date: S, headline: S, body: S) -> Self {
        Event {
            date: NaiveDate::parse_from_str(date.to_string().as_str(), "%Y-%m-%d").expect("date"),
            headline: headline.to_string(),
            body: body.to_string(),
        }
    }
}
