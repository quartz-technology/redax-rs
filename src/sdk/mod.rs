use crate::relay::RelayClient;
use crate::sdk::data::DataSDK;

pub mod data;

pub struct RelaySDK {
    data: DataSDK,
}

impl RelaySDK {
    #[allow(dead_code)]
    pub fn new(client: RelayClient) -> Self {
        RelaySDK {
            data: DataSDK::new(client),
        }
    }

    pub fn data(self) -> DataSDK {
        self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_the_relay_data_sdk() {
        let client = RelayClient::new().unwrap();
        let _sdk = RelaySDK::new(client);
    }
}
