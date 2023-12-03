use alloy_primitives::BlockNumber;
use mev_rs::types::BidTrace;
use serde_aux::prelude::*;

#[derive(Debug, serde::Deserialize)]
pub struct BidDelivered {
    #[serde(flatten)]
    pub bid_trace: BidTrace,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub block_number: BlockNumber,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub num_tx: u32,
}

#[derive(Debug, serde::Deserialize)]
pub struct BidReceived {
    #[serde(flatten)]
    pub bid_trace: BidTrace,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub block_number: BlockNumber,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub num_tx: u32,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub timestamp: u64,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub timestamp_ms: u64,

    pub optimistic_submission: bool,
}
