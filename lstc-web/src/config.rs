#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub api_url: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            api_url: "https://0d99uja6bd.execute-api.eu-west-2.amazonaws.com/production/event/2024"
                .to_string(),
        }
    }
}
