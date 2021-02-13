use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum Network {
    Bitcoin,
    Testnet,
    Liquid,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct Request {
    pub network: Network,
    pub transaction: Transaction,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct Transaction(pub Vec<u8>);

impl Network {
    pub fn api_endpoint(&self) -> &str {
        match self {
            Network::Bitcoin => "https://blockstream.info/api/tx",
            Network::Testnet => "https://blockstream.info/testnet/api/tx",
            Network::Liquid => "https://blockstream.info/liquid/api/tx",
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct UnknownNetworkError(String);

impl Error for UnknownNetworkError {}

impl Display for UnknownNetworkError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unknown network \"{}\"", self.0)
    }
}

impl FromStr for Network {
    type Err = UnknownNetworkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bitcoin" => Ok(Network::Bitcoin),
            "testnet" => Ok(Network::Testnet),
            "liquid" => Ok(Network::Liquid),
            // Annoying that we need to clone here, but `FromStr::Err` doesn't allow lifetimes
            other => Err(UnknownNetworkError(other.to_string())),
        }
    }
}

impl FromStr for Transaction {
    type Err = hex::FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Transaction(hex::decode(s)?))
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Why no `encode_to_fmt`???
        f.write_str(&hex::encode(&self.0))
    }
}
