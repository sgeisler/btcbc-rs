#![feature(async_closure)]

use btcbc::{Network, Request, Transaction};
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use nym_websocket::responses::ServerResponse;
use structopt::StructOpt;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tracing::{debug, error, info, warn};
use tracing_subscriber::EnvFilter;

#[derive(StructOpt)]
struct Options {
    #[structopt(short, long, default_value = "ws://127.0.0.1:1977")]
    websocket: String,
}

async fn submit_tx(net: Network, tx: Transaction) {
    debug!("Submitting transaction to network {:?}: {}", net, tx);

    let client = reqwest::Client::new();
    match client
        .post(net.api_endpoint())
        .body(tx.to_string())
        .send()
        .await
    {
        Ok(response) => {
            debug!("Done. Response: {:?}", response.text().await);
        }
        Err(e) => {
            error!("Error submitting tx: {}", e);
        }
    }
}

fn build_identity_request() -> Message {
    let nym_message = nym_websocket::requests::ClientRequest::SelfAddress;
    Message::Binary(nym_message.serialize())
}

fn parse_nym_message(msg: Message) -> nym_websocket::responses::ServerResponse {
    match msg {
        Message::Binary(bytes) => nym_websocket::responses::ServerResponse::deserialize(&bytes)
            .expect("Could not decode nym client response"),
        msg => panic!("Unexpected message: {:?}", msg),
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let options: Options = Options::from_args();

    debug!("Connecting to websocket at {}", &options.websocket);
    let (mut ws, _) = connect_async(&options.websocket)
        .await
        .expect("Couldn't connect to nym websocket");

    debug!("Requesting own identity from nym client");
    ws.send(build_identity_request())
        .await
        .expect("failed to send identity request");

    while let Some(Ok(msg)) = ws.next().await {
        let msg = parse_nym_message(msg);

        let msg_bytes = match msg {
            ServerResponse::Received(msg_bytes) => {
                debug!("Received client request {:?}", msg_bytes);
                msg_bytes
            }
            ServerResponse::SelfAddress(addr) => {
                info!("Listening on {}", addr);
                continue;
            }
            ServerResponse::Error(err) => {
                error!("Received error from nym client: {}", err);
                continue;
            }
        };

        let request: Request = match bincode::deserialize(&msg_bytes.message) {
            Ok(msg) => msg,
            Err(e) => {
                warn!("Could not decode client request");
                debug!("Client request decoding error: {}", e);
                continue;
            }
        };

        tokio::spawn(submit_tx(request.network, request.transaction));
    }
}
