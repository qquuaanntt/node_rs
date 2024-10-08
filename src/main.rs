use alloy_provider::network::Ethereum;
use alloy_provider::Provider;
use alloy_provider::RootProvider;
use alloy_provider::WsConnect;
use alloy_pubsub::PubSubFrontend;
use alloy_rpc_client::ClientBuilder;
use alloy_rpc_types::Transaction;
use futures::StreamExt;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv::dotenv().ok();

    println!("sourced .env");

    let builder = ClientBuilder::default()
        .ws(WsConnect::new(
            &std::env::var("WS_URL").expect("WS_URL not in .env"),
        ))
        .await?;

    println!("made builder");

    let client: RootProvider<PubSubFrontend, Ethereum> = RootProvider::new(builder);

    println!("made client");

    let sub = client.subscribe_full_pending_transactions().await?;

    println!("made subscription");

    let mut stream: alloy_pubsub::SubscriptionStream<Transaction> = sub.into_stream();

    println!("made stream");

    while let Some(tx) = stream.next().await {
        println!("found new transaction: {:?}", tx.hash);
    }

    Ok(())
}
