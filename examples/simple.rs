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
