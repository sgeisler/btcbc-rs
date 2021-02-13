#![feature(async_closure)]

use structopt::StructOpt;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use futures::sink::SinkExt;
use futures::stream::StreamExt;

#[derive(StructOpt)]
struct Options {
    websocket: String
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Network {
    Bitcoin,
    Testnet,
    Liquid,
}

impl Network {
    fn api_endpoint(&self) -> &str {
        match self {
            Network::Bitcoin => "https://blockstream.info/api/tx",
            Network::Testnet => "https://blockstream.info/testnet/api/tx",
            Network::Liquid => "https://blockstream.info/liquid/api/tx",
        }
    }
}

async fn submit_tx(net: Network, tx: String) {
    println!("Submitting transaction to network {:?}: {}", net, tx);

    let client = reqwest::Client::new();
    let res = client.post(net.api_endpoint())
        .body(tx)
        .send()
        .await;

    println!("Done. Response: {:?}", res.unwrap().text().await);
}

#[tokio::main]
async fn main() {
    let options: Options = Options::from_args();
    let (mut ws, _) = connect_async(&options.websocket)
        .await
        .expect("Couldn't connect to nym websocket");

    ws.send(Message::text("{\"type\": \"selfAddress\"}")).await.unwrap();
    println!("{:?}", ws.next().await.unwrap());

    while let Some(Ok(msg)) = ws.next().await {
        println!("Received {:?}", msg);
        let command = match msg {
            Message::Text(command) => command,
            Message::Binary(bin) => {
                if let Ok(s) = String::from_utf8(bin) {
                    s
                } else {
                    println!("Invalid string");
                    continue;
                }
            }
            Message::Close(_) => {
                println!("Connection closed");
                return;
            }
            msg => {
                println!("Received unsupported message: {:?}", msg);
                continue;
            }
        };

        let parts = command.split(':').collect::<Vec<_>>();
        let (net, tx) = if parts.len() == 1 {
            (Network::Bitcoin, parts[0])
        } else if parts.len() == 2 {
            match parts[0] {
                "testnet" => (Network::Testnet, parts[1]),
                "liquid" => (Network::Liquid, parts[1]),
                net => {
                    println!("Unknown network: {}", net);
                    continue;
                }
            }
        } else {
            println!("Unsupported command: {:?}", command);
            continue;
        };

        tokio::spawn(submit_tx(net, tx.into()));
    }
}
