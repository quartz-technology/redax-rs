use crate::relay::RelayClient;
use crate::sdk::data::v1::DataV1SDK;

pub mod v1;

pub struct DataSDK {
    v1: DataV1SDK,
}

impl DataSDK {
    pub fn new(client: RelayClient) -> DataSDK {
        DataSDK {
            v1: DataV1SDK::new(client),
        }
    }

    pub fn v1(self) -> DataV1SDK {
        self.v1
    }
}
