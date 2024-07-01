use aws_config::SdkConfig;
use aws_sdk_dynamodb::{types::AttributeValue, Client};
use chrono::NaiveDate;
use lstc_domain::Event;
use std::{collections::HashMap, marker::PhantomData};

pub trait Record {
    fn pk(&self) -> String;
    fn sk(&self) -> String;
    fn from_hashmap(record: &HashMap<String, AttributeValue>) -> Self;
    fn to_hashmap(&self) -> HashMap<String, AttributeValue>;
}

impl Record for Event {
    fn pk(&self) -> String {
        self.date.format("E#%Y").to_string()
    }

    fn sk(&self) -> String {
        self.date.format("%Y-%m-%d").to_string()
    }

    fn from_hashmap(record: &HashMap<String, AttributeValue>) -> Self {
        Event {
            date: NaiveDate::parse_from_str(record.get("sk").unwrap().as_s().unwrap(), "%Y-%m-%d")
                .unwrap(),
            title: record.get("title").unwrap().as_s().unwrap().to_string(),
            sub_title: record.get("sub_title").unwrap().as_s().unwrap().to_string(),
            body: record.get("body").unwrap().as_s().unwrap().to_string(),
        }
    }

    fn to_hashmap(&self) -> HashMap<String, AttributeValue> {
        let mut result = HashMap::new();
        result.insert(
            "pk".to_string(),
            AttributeValue::S(self.date.format("E#%Y").to_string()),
        );
        result.insert(
            "sk".to_string(),
            AttributeValue::S(self.date.format("%Y-%m-%d").to_string()),
        );
        result.insert("title".to_string(), AttributeValue::S(self.title.clone()));
        result.insert(
            "sub_title".to_string(),
            AttributeValue::S(self.sub_title.clone()),
        );
        result.insert("body".to_string(), AttributeValue::S(self.body.clone()));
        result
    }
}

pub struct Repository<T>
where
    T: Record,
{
    client: Client,
    table_name: String,
    _t: PhantomData<T>,
}

impl<T: Record> Repository<T> {
    pub fn new(config: &SdkConfig, table_name: String) -> Self {
        Self {
            client: Client::new(config),
            table_name: table_name.clone(),
            _t: PhantomData,
        }
    }

    pub async fn save(&self, entity: T) {
        let record = entity.to_hashmap();
        self.client
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(record))
            .send()
            .await
            .unwrap();
    }

    pub async fn load(&self, pk: String, sk: String) -> Option<T> {
        let response = self
            .client
            .get_item()
            .table_name(&self.table_name)
            .key("pk", AttributeValue::S(pk))
            .key("sk", AttributeValue::S(sk))
            .send()
            .await
            .unwrap();
        response.item().map(T::from_hashmap)
    }

    pub async fn query(&self, pk: String) -> Vec<T> {
        let response = self
            .client
            .query()
            .table_name(&self.table_name)
            .key_condition_expression("#pk = :pk")
            .expression_attribute_names("#pk", "pk")
            .expression_attribute_values(":pk", AttributeValue::S(pk))
            .send()
            .await
            .unwrap();
        response
            .items
            .unwrap()
            .iter()
            .map(T::from_hashmap)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::repository::Record;
    use aws_config::BehaviorVersion;
    use aws_sdk_dynamodb::types::AttributeValue;
    use chrono::NaiveDate;
    use lstc_domain::Event;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test() {
        let config = aws_config::load_defaults(BehaviorVersion::v2024_03_28()).await;

        let repo = Repository::new(&config, "lstc_website--www--data".to_string());

        repo.save(Event::new("2024-01-01", "Jan", "Jan", "")).await;
        repo.save(Event::new("2024-02-01", "Feb", "Feb", "")).await;
        repo.save(Event::new("2024-03-01", "Mar", "Mar", "")).await;

        let item = repo
            .load("E#2024".to_string(), "2024-01-01".to_string())
            .await
            .unwrap();
        println!("> {} {} {}", item.date, item.title, item.sub_title);

        let items = repo.query("E#2024".to_string()).await;
        println!("Items");
        for item in items {
            println!("> {} {} {}", item.date, item.title, item.sub_title);
        }
        println!("Done");
    }

    #[test]
    fn convert_to_hashmap() {
        let subject = Event::new("2024-02-01", "title", "sub_title", "body");
        let mut expected = HashMap::<String, AttributeValue>::new();
        expected.insert("pk".to_string(), AttributeValue::S("E#2024".to_string()));
        expected.insert(
            "sk".to_string(),
            AttributeValue::S("2024-02-01".to_string()),
        );
        expected.insert("title".to_string(), AttributeValue::S("title".to_string()));
        expected.insert(
            "sub_title".to_string(),
            AttributeValue::S("sub_title".to_string()),
        );
        expected.insert("body".to_string(), AttributeValue::S("body".to_string()));

        let actual = subject.to_hashmap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn convert_from_hashmap() {
        let mut subject = HashMap::<String, AttributeValue>::new();
        subject.insert("pk".to_string(), AttributeValue::S("E#2024".to_string()));
        subject.insert(
            "sk".to_string(),
            AttributeValue::S("2024-02-01".to_string()),
        );
        subject.insert("title".to_string(), AttributeValue::S("title".to_string()));
        subject.insert(
            "sub_title".to_string(),
            AttributeValue::S("sub_title".to_string()),
        );
        subject.insert("body".to_string(), AttributeValue::S("body".to_string()));

        let actual = Event::from_hashmap(&subject);

        assert_eq!(actual.date, NaiveDate::from_ymd_opt(2024, 2, 1).unwrap());
        assert_eq!(actual.title, "title".to_string());
        assert_eq!(actual.sub_title, "sub_title".to_string());
        assert_eq!(actual.body, "body".to_string());
    }

    #[test]
    fn pk_and_sk() {
        let subject = Event::new("2024-02-01", "title", "sub_title", "body");

        assert_eq!(Record::pk(&subject), "E#2024");
        assert_eq!(Record::sk(&subject), "2024-02-01");
    }
}
