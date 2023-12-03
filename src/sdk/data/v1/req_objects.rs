use alloy_primitives::{BlockHash, BlockNumber};
use ethereum_consensus::primitives::{BlsPublicKey, Slot};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GetBidsDeliveredRequestError {
    #[error("conflicting params, cannot specify both slot and cursor")]
    ConflictingParams,
}

#[derive(Debug, PartialEq)]
pub enum ResultsOrder {
    IncreasingValue,
    DecreasingValue,
}

impl ResultsOrder {
    fn as_str(&self) -> &'static str {
        match self {
            ResultsOrder::IncreasingValue => "value",
            ResultsOrder::DecreasingValue => "-value",
        }
    }
}

#[derive(Default)]
pub struct GetBidsDeliveredRequest {
    slot: Option<Slot>,
    cursor: Option<u32>,
    limit: Option<u32>,
    block_hash: Option<BlockHash>,
    block_number: Option<BlockNumber>,
    proposer_public_key: Option<BlsPublicKey>,
    builder_public_key: Option<BlsPublicKey>,
    order: Option<ResultsOrder>,
}

impl GetBidsDeliveredRequest {
    pub fn with_slot(mut self, slot: Slot) -> Self {
        self.slot = Some(slot);

        self
    }

    pub fn with_cursor(mut self, cursor: u32) -> Self {
        self.cursor = Some(cursor);

        self
    }

    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);

        self
    }

    pub fn with_block_hash(mut self, block_hash: BlockHash) -> Self {
        self.block_hash = Some(block_hash);

        self
    }

    pub fn with_block_number(mut self, block_number: BlockNumber) -> Self {
        self.block_number = Some(block_number);

        self
    }

    pub fn with_proposer_public_key(mut self, ppk: BlsPublicKey) -> Self {
        self.proposer_public_key = Some(ppk);

        self
    }

    pub fn with_builder_public_key(mut self, bpk: BlsPublicKey) -> Self {
        self.builder_public_key = Some(bpk);

        self
    }

    pub fn with_order(mut self, order: ResultsOrder) -> Self {
        self.order = Some(order);

        self
    }

    pub fn query_params(self) -> HashMap<String, String> {
        let mut query_params = HashMap::new();

        if let Some(slot) = self.slot {
            query_params.insert("slot".to_owned(), slot.to_string());
        }

        if let Some(cursor) = self.cursor {
            query_params.insert("cursor".to_owned(), cursor.to_string());
        }

        if let Some(limit) = self.limit {
            query_params.insert("limit".to_owned(), limit.to_string());
        }

        if let Some(block_hash) = self.block_hash {
            query_params.insert("block_hash".to_owned(), block_hash.to_string());
        }

        if let Some(block_number) = self.block_number {
            query_params.insert("block_number".to_owned(), block_number.to_string());
        }

        if let Some(proposer_public_key) = self.proposer_public_key {
            let vpk = "0x".to_owned() + hex::encode(proposer_public_key.as_slice()).as_str();
            query_params.insert("proposer_pubkey".to_owned(), vpk);
        }

        if let Some(builder_public_key) = self.builder_public_key {
            let bpk = "0x".to_owned() + hex::encode(builder_public_key.as_slice()).as_str();
            query_params.insert("builder_pubkey".to_owned(), bpk);
        }

        if let Some(order) = self.order {
            query_params.insert("order_by".to_owned(), order.as_str().to_string());
        }

        query_params
    }

    pub fn validate(&self) -> Result<(), GetBidsDeliveredRequestError> {
        if self.cursor.is_some() && self.cursor.is_some() {
            return Err(GetBidsDeliveredRequestError::ConflictingParams);
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum GetBidsReceivedRequestError {
    #[error("need to query for specific slot or block_hash or block_number or builder_pubkey")]
    MissingMandatoryParam,
}

#[derive(Default)]
pub struct GetBidsReceivedRequest {
    slot: Option<Slot>,
    block_hash: Option<BlockHash>,
    block_number: Option<BlockNumber>,
    builder_public_key: Option<BlsPublicKey>,
    limit: Option<u32>,
}

impl GetBidsReceivedRequest {
    pub fn with_slot(mut self, slot: Slot) -> Self {
        self.slot = Some(slot);

        self
    }

    pub fn with_block_hash(mut self, block_hash: BlockHash) -> Self {
        self.block_hash = Some(block_hash);

        self
    }

    pub fn with_block_number(mut self, block_number: BlockNumber) -> Self {
        self.block_number = Some(block_number);

        self
    }

    pub fn with_builder_public_key(mut self, bpk: BlsPublicKey) -> Self {
        self.builder_public_key = Some(bpk);

        self
    }

    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);

        self
    }

    pub fn query_params(self) -> HashMap<String, String> {
        let mut query_params = HashMap::new();

        if let Some(slot) = self.slot {
            query_params.insert("slot".to_owned(), slot.to_string());
        }

        if let Some(block_hash) = self.block_hash {
            query_params.insert("block_hash".to_owned(), block_hash.to_string());
        }

        if let Some(block_number) = self.block_number {
            query_params.insert("block_number".to_owned(), block_number.to_string());
        }

        if let Some(builder_public_key) = self.builder_public_key {
            let bpk = "0x".to_owned() + hex::encode(builder_public_key.as_slice()).as_str();
            query_params.insert("builder_pubkey".to_owned(), bpk);
        }

        if let Some(limit) = self.limit {
            query_params.insert("limit".to_owned(), limit.to_string());
        }

        query_params
    }

    pub fn validate(&self) -> Result<(), GetBidsReceivedRequestError> {
        match self.slot.is_none()
            && self.block_hash.is_none()
            && self.block_number.is_none()
            && self.builder_public_key.is_none()
        {
            true => Err(GetBidsReceivedRequestError::MissingMandatoryParam),
            false => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_default_get_bids_delivered_request_params() {
        let params = GetBidsDeliveredRequest::default();

        assert_eq!(None, params.slot);
        assert_eq!(None, params.cursor);
        assert_eq!(None, params.limit);
        assert_eq!(None, params.block_hash);
        assert_eq!(None, params.block_number);
        assert_eq!(None, params.proposer_public_key);
        assert_eq!(None, params.builder_public_key);
        assert_eq!(None, params.order);
    }

    #[test]
    fn it_builds_get_bids_delivered_request_params_with_values() {
        let params = GetBidsDeliveredRequest::default()
            .with_slot(1)
            .with_cursor(2)
            .with_limit(3)
            .with_block_hash(BlockHash::default())
            .with_block_number(4)
            .with_proposer_public_key(BlsPublicKey::default())
            .with_builder_public_key(BlsPublicKey::default())
            .with_order(ResultsOrder::IncreasingValue);

        assert_eq!(Some(1), params.slot);
        assert_eq!(Some(2), params.cursor);
        assert_eq!(Some(3), params.limit);
        assert_eq!(Some(BlockHash::default()), params.block_hash);
        assert_eq!(Some(4), params.block_number);
        assert_eq!(Some(BlsPublicKey::default()), params.proposer_public_key);
        assert_eq!(Some(BlsPublicKey::default()), params.builder_public_key);
        assert_eq!(Some(ResultsOrder::IncreasingValue), params.order);
    }

    #[test]
    fn it_converts_get_bids_delivered_to_query_params() {
        let query_params = GetBidsDeliveredRequest::default()
            .with_slot(1)
            .with_cursor(2)
            .with_limit(3)
            .with_block_hash(BlockHash::default())
            .with_block_number(4)
            .with_proposer_public_key(BlsPublicKey::default())
            .with_builder_public_key(BlsPublicKey::default())
            .with_order(ResultsOrder::IncreasingValue)
            .query_params();

        assert_eq!("1", query_params.get("slot").expect("TODO").as_str());
        assert_eq!("2", query_params.get("cursor").expect("TODO").as_str());
        assert_eq!("3", query_params.get("limit").expect("TODO").as_str());
        assert_eq!(
            "0x0000000000000000000000000000000000000000000000000000000000000000",
            query_params.get("block_hash").expect("TODO").as_str()
        );
        assert_eq!(
            "4",
            query_params.get("block_number").expect("TODO").as_str()
        );
        assert_eq!("0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000", query_params.get("proposer_pubkey").expect("TODO").as_str());
        assert_eq!("0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000", query_params.get("builder_pubkey").expect("TODO").as_str());
        assert_eq!(
            "value",
            query_params.get("order_by").expect("TODO").as_str()
        );
    }
}
