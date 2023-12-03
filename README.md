# redax-rs
Rust SDK for the Relay Data Transparency API on Ethereum.


## Getting Started

To install the SDK in your own go project, run the following command:
```shell
# Coming soon!
```

### Examples

_More examples will come soon ! You can find the existing examples in the [`examples`](./examples) 
directory of this repository._

Below is an example of the SDK usage with the default HTTP client:
```rust
use redax_rs::sdk::data::v1::GetBidsDeliveredRequest;
use redax_rs::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let clt = relay::RelayClient::new()?.with_api_url("https://boost-relay.flashbots.net")?;
    let relay_sdk = sdk::RelaySDK::new(clt);

    // Get the bids delivered.
    let bids_delivered = relay_sdk
        .data()
        .v1()
        .get_bids_delivered(GetBidsDeliveredRequest::default())
        .await?;

    // Print the first bid's value.
    println!("{:?}", bids_delivered[0].bid_trace.value.to_string());

    Ok(())
}
```

### API

Below is a list of the supported API endpoints:

| Name       	                                            | Status 	 |
|:--------------------------------------------------------|:--------:|
| `/relay/v1/data/bidtraces/proposer_payload_delivered` 	 |  ✅   	   |
| `/relay/v1/data/bidtraces/builder_blocks_received` 	    |  ✅   	   |
| `/relay/v1/data/validator_registration` 	               |  ✅   	   |

## Issues

This SDK is still under active development, if you find any bug or have a feature request please
submit an appropriate issue [here](https://github.com/quartz-technology/redax-rs/issues/new/choose).

## Contributing

If you would like to contribute to this project, please refer to the instructions in the
dedicated document [here](./CONTRIBUTING.md).

## Authors

This project is a pure open-source contribution to the Ethereum ecosystem.
It is currently maintained by the 🤖 at [Quartz Technology](https://github.com/quartz-technology).
