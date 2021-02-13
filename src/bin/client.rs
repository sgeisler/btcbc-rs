use structopt::StructOpt;
use nym_addressing::clients::Recipient;
use btcbc::Network;

#[derive(StructOpt)]
struct Options {
    #[structopt(parse(try_from_str = Recipient::try_from_base58_string))]
    service_provider: Recipient,
    network: Network,
    transaction: String,
}

fn main() {}