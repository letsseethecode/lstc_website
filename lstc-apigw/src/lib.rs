use std::{collections::HashMap, marker::PhantomData};

// use aws_config::SdkConfig;
// use aws_sdk_dynamodb::{types::AttributeValue, Client};
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
    pub title: String,
    pub sub_title: String,
    pub body: String,
}

impl Event {
    pub fn new<S: ToString>(date: S, title: S, sub_title: S, body: S) -> Self {
        Event {
            date: NaiveDate::parse_from_str(date.to_string().as_str(), "%Y-%m-%d").expect("date"),
            title: title.to_string(),
            sub_title: sub_title.to_string(),
            body: body.to_string(),
        }
    }
}

// impl From<&HashMap<String, AttributeValue>> for Event {
//     fn from(value: &HashMap<String, AttributeValue>) -> Self {
//         Event {
//             date: NaiveDate::parse_from_str(value.get("sk").unwrap().as_s().unwrap(), "%Y-%m-%d")
//                 .unwrap(),
//             title: value.get("title").unwrap().as_s().unwrap().to_string(),
//             sub_title: value.get("sub_title").unwrap().as_s().unwrap().to_string(),
//             body: value
//                 .get("body")
//                 .map_or("", |v| v.as_s().unwrap())
//                 .to_string(),
//         }
//     }
// }

// impl From<Event> for HashMap<String, AttributeValue> {
//     fn from(value: Event) -> Self {
//         let mut result = HashMap::new();
//         result.insert(
//             "pk".to_string(),
//             AttributeValue::S(value.date.format("E#%Y").to_string()),
//         );
//         result.insert(
//             "sk".to_string(),
//             AttributeValue::S(value.date.format("%YYYY-%mm-%dd").to_string()),
//         );
//         result.insert("title".to_string(), AttributeValue::S(value.title));
//         result.insert("sub_title".to_string(), AttributeValue::S(value.sub_title));
//         result.insert("body".to_string(), AttributeValue::S(value.body));
//         result
//     }
// }

// pub struct Repository<T> {
//     table_name: String,
//     client: Client,
//     _t: PhantomData<T>,
// }

// impl<T> Repository<T> {
//     pub fn new(config: &SdkConfig, table_name: String) -> Self {
//         let client = Client::new(config);
//         Self {
//             client,
//             table_name,
//             _t: PhantomData,
//         }
//     }

//     pub async fn save(entity: &T) {
//         todo!();
//     }

//     pub async fn delete(pk: String, sk: String) {
//         todo!();
//     }

//     pub async fn load(pk: String, sk: String) -> T {
//         todo!();
//     }

//     pub async fn query(pk: String) -> Vec<T> {
//         todo!();
//     }
// }
