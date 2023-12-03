use crate::relay::{RelayClient, RelayClientError, Request};
use crate::sdk::data::v1::req_objects::{
    GetBidsDeliveredRequest, GetBidsDeliveredRequestError, GetBidsReceivedRequest,
    GetBidsReceivedRequestError,
};
use crate::sdk::data::v1::res_objects::{BidDelivered, BidReceived};
use ethereum_consensus::builder::SignedValidatorRegistration;
use ethereum_consensus::primitives::BlsPublicKey;
use reqwest::Method;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DataV1SDKError {
    #[error(transparent)]
    ErrGetBidsDeliveredRequest(#[from] GetBidsDeliveredRequestError),

    #[error(transparent)]
    ErrGetBidsReceivedRequest(#[from] GetBidsReceivedRequestError),

    #[error(transparent)]
    Client(#[from] RelayClientError),

    #[error("failed to deserialize response")]
    ResponseDeserializationError(#[from] reqwest::Error),
}

pub struct DataV1SDK {
    client: RelayClient,
}

impl DataV1SDK {
    pub fn new(client: RelayClient) -> Self {
        DataV1SDK { client }
    }

    pub async fn get_bids_delivered(
        &self,
        params: GetBidsDeliveredRequest,
    ) -> Result<Vec<BidDelivered>, DataV1SDKError> {
        params.validate()?;

        let query_params = params.query_params();

        let req = Request {
            method: Method::GET,
            path: "/relay/v1/data/bidtraces/proposer_payload_delivered".to_string(),
            query_params,
        };

        let res = self.client.do_request(req).await?;
        let bids_delivered = res
            .json::<Vec<BidDelivered>>()
            .await
            .map_err(DataV1SDKError::ResponseDeserializationError)?;

        Ok(bids_delivered)
    }

    pub async fn get_bids_received(
        &self,
        params: GetBidsReceivedRequest,
    ) -> Result<Vec<BidReceived>, DataV1SDKError> {
        params.validate()?;

        let query_params = params.query_params();

        let req = Request {
            method: Method::GET,
            path: "/relay/v1/data/bidtraces/builder_blocks_received".to_string(),
            query_params,
        };

        let res = self.client.do_request(req).await?;
        let bids_received = res
            .json::<Vec<BidReceived>>()
            .await
            .map_err(DataV1SDKError::ResponseDeserializationError)?;

        Ok(bids_received)
    }

    pub async fn get_validators_registration(
        &self,
        validator_public_key: &BlsPublicKey,
    ) -> Result<SignedValidatorRegistration, DataV1SDKError> {
        let vpk = "0x".to_owned() + hex::encode(validator_public_key.as_slice()).as_str();

        let mut query_params = HashMap::new();
        query_params.insert("pubkey".to_string(), vpk);

        let req = Request {
            method: Method::GET,
            path: "/relay/v1/data/validator_registration".to_string(),
            query_params,
        };

        let res = self.client.do_request(req).await?;
        let svr = res
            .json::<SignedValidatorRegistration>()
            .await
            .map_err(DataV1SDKError::ResponseDeserializationError)?;

        Ok(svr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_relay_data_v1_sdk() {
        let client = RelayClient::new().unwrap();
        DataV1SDK::new(client);
    }

    #[tokio::test]
    async fn it_gets_existing_validators_registration() {
        let client = RelayClient::new()
            .unwrap()
            .with_api_url("https://boost-relay.flashbots.net")
            .unwrap();

        let data_v1_sdk = DataV1SDK::new(client);

        let validator = hex::decode("84fd071ff440e9f466e367cbd753714ce3c69959fe0d985a7454a6de6ea410a2aa27692ab39599470d0a5a876126f563")
            .expect("TODO");

        let vpk = BlsPublicKey::try_from(validator.as_slice()).expect("TODO");
        let res = data_v1_sdk.get_validators_registration(&vpk).await;

        assert_eq!(true, res.is_ok())
    }

    #[tokio::test]
    async fn it_gets_bids_delivered() {
        let client = RelayClient::new()
            .unwrap()
            .with_api_url("https://boost-relay.flashbots.net")
            .unwrap();

        let data_v1_sdk = DataV1SDK::new(client);
        let res = data_v1_sdk
            .get_bids_delivered(GetBidsDeliveredRequest::default())
            .await;

        assert_eq!(true, res.is_ok())
    }

    #[tokio::test]
    async fn it_gets_bids_received() {
        let client = RelayClient::new()
            .unwrap()
            .with_api_url("https://boost-relay.flashbots.net")
            .unwrap();

        let data_v1_sdk = DataV1SDK::new(client);
        let res = data_v1_sdk
            .get_bids_received(GetBidsReceivedRequest::default().with_slot(7_898_580))
            .await;

        assert_eq!(true, res.is_ok())
    }
}
