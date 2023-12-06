use payments::bitcoin_client::BitcoinClient;
use payments::BtcPaymentRequest;

pub mod payments {
    tonic::include_proto!("payments");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BitcoinClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(BtcPaymentRequest {
        from_addr: "1Gokm82v6DmtwKEB8AiVhm82hyFSsEvBDK".into(),
        to_addr: "1F5ySj7X3JwWK7R9mLgRXcBvJv5cXh6u7".into(),
        amount: 22,
    });

    let response = client.send_payment(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
