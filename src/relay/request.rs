use reqwest::Method;
use std::collections::HashMap;
use url::{ParseError, Url};

pub struct Request {
    pub method: Method,
    pub path: String,
    pub query_params: HashMap<String, String>,
}

impl Request {
    pub fn get_url(&self, api_url: &str) -> Result<Url, ParseError> {
        let mut url = Url::parse((api_url.to_string() + self.path.as_str()).as_str())?;

        self.query_params.iter().for_each(|(key, value)| {
            url.set_query(Some(format!("{}={}", key, value).as_str()));
        });

        Ok(url)
    }
}
