use crate::relay::Request;
use reqwest::{Client, ClientBuilder, Response, StatusCode};
use thiserror::Error;
use url::{ParseError, Url};

#[derive(Debug, Error)]
pub enum RelayClientError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error("failed to parse api url")]
    ApiUrlParseError(#[from] ParseError),

    #[error("api url must not have a trailing slash")]
    ApiUrlHasTrailingSlash,

    #[error("got error in response")]
    ResponseError { status: StatusCode, data: String },
}

pub struct RelayClient {
    pub api_url: String,
    pub requester: Client,
}

impl RelayClient {
    pub fn new() -> Result<Self, RelayClientError> {
        let builder = ClientBuilder::new();
        let requester = builder.build()?;

        Ok(RelayClient {
            api_url: "http://localhost:18550".to_string(),
            requester,
        })
    }

    pub fn with_api_url(mut self, api_url: &str) -> Result<Self, RelayClientError> {
        Url::parse(api_url)?;

        match api_url.ends_with('/') {
            true => Err(RelayClientError::ApiUrlHasTrailingSlash),
            false => {
                self.api_url = api_url.to_owned();
                Ok(self)
            }
        }
    }

    pub fn with_requester(mut self, requester: Client) -> Self {
        self.requester = requester;
        self
    }

    pub async fn do_request(&self, req: Request) -> Result<Response, RelayClientError> {
        let url = req.get_url(self.api_url.as_str())?;
        let http_req = reqwest::Request::new(req.method, url);
        let res = self.requester.execute(http_req).await?;

        match res.status().as_u16() < 200 || res.status().as_u16() >= 300 {
            true => {
                let status = res.status();
                let data = res.text().await?;

                Err(RelayClientError::ResponseError { status, data })
            }
            false => Ok(res),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_relay_data_client() {
        let client = RelayClient::new();

        assert_eq!(true, client.is_ok());
    }

    #[test]
    fn it_builds_relay_data_client_with_api_url() {
        let client = RelayClient::new()
            .unwrap()
            .with_api_url("https://boost-relay.flashbots.net");

        assert_eq!(true, client.is_ok());
    }

    #[test]
    fn it_fails_to_build_relay_data_client_with_invalid_api_url() {
        let client = RelayClient::new().unwrap().with_api_url("");

        assert_eq!(true, client.is_err());
    }

    #[test]
    fn it_fails_to_build_relay_data_client_with_malformed_api_url() {
        let client = RelayClient::new()
            .unwrap()
            .with_api_url("https://boost-relay.flashbots.net/");

        assert_eq!(true, client.is_err());
    }
}
