use alloy_primitives::address;
use alloy_primitives::Address;
use alloy_provider::network::Ethereum;
use alloy_provider::Provider;
use alloy_provider::RootProvider;
use alloy_provider::WsConnect;
use alloy_pubsub::PubSubFrontend;
use alloy_rpc_client::ClientBuilder;
use alloy_rpc_types::Transaction;
use alloy_sol_macro::sol;
use alloy_sol_types::SolCall;
use futures::StreamExt;

const UNI_V2_ROUTER: Address = address!("3fC91A3afd70395Cd496C647d5a6CC9D4B2b7FAD");

sol!(UniV2Router, "src/v2_router.json");

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv::dotenv().ok();

    run().await
}

async fn run() -> eyre::Result<()> {
    let builder = ClientBuilder::default()
        .ws(WsConnect::new(
            &std::env::var("WS_URL").expect("WS_URL not in .env"),
        ))
        .await?;

    let client: RootProvider<PubSubFrontend, Ethereum> = RootProvider::new(builder);

    let sub = client.subscribe_full_pending_transactions().await?;

    let mut stream: alloy_pubsub::SubscriptionStream<Transaction> = sub.into_stream();

    while let Some(tx) = stream.next().await {
        decode_tx(tx);
    }

    Ok(())
}

fn decode_tx(tx: Transaction) {
    if tx.to == Some(UNI_V2_ROUTER) {
        if tx.input.len() >= 4 {
            let func_sig = &tx.input[..4];

            if func_sig == UniV2Router::swapExactETHForTokensCall::SELECTOR {
                let data = &*tx.input.0;
                let call =
                    UniV2Router::swapETHForExactTokensCall::abi_decode_raw(&data[4..], false)
                        .unwrap();

                println!(
                    "\n\nFOUND TRANSACTON:\n\tamountOut: {}\n\tdeadline: {}\n\tto: {}\n\tpath: {:?}\n\n",
                    call.amountOut, call.deadline, call.to, call.path
                );
            } else {
                println!(
                    "transaction call is not swapExactETHForTokensCall: {:?} -- {:?}",
                    tx.hash, func_sig
                );
            }
        } else {
            println!(
                "transaction call is not swapExactETHForTokensCall: {:?}",
                tx.hash
            );
        }
    } else {
        println!("transaction not to V2 Router: {:?}", tx.hash);
    }
}
