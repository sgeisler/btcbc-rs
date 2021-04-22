# Nym Bitcoin Broadcaster
This repository implement an anonymous Bitcoin transaction broadcasting tool on top of
[Nym](https://github.com/nymtech/nym), a mixnet still under heavy development. So while the technology looks promising
any anonymity claims are to be taken with a grain of salt. This project is provided as-is, it might work as expected or
not, please don't rely on it without vetting it yourself.

There are two parts:
* **Client:** connects to a nym native-client and sends a transaction to a specified server (aka service provider)
* **Server:** listens for incoming nym packets from its nym native-client. If they are valid client requests containing
a transaction it is broadcasted to its respective network using [blockstream.info](https://blockstream.info).

## Usage
### Nym Native Client
To use either one you have to initialize and run a [Nym client](https://nymtech.net/docs/build-peapps/native-client/):

```bash
nym-client init --id client
nym-client run --id client
```

If you want to run both client and server on one machine it's advisable to run two nym clients on different ports:

```bash
nym-client init --id client # default port = 1977
nym-client init --id server --port 1978

nym-client run --id client
nym-client run --id server
``` 

### BTC-BC Client
```
btcbc 0.1.0

USAGE:
    client [OPTIONS] <transaction>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --network <network>                      one of 'bitcoin', 'testnet' or 'liquid' [default: bitcoin]
    -s, --service-provider <service-provider>
             [default:
            7GAmWTUr3wude4LkRBJ78UmD2QMCgQvr8RCRJHW1fUYf.DHhHL8ZcnEEFq3UKuD7E31aWdnzuWdeJv1wRicj9n6tU@AmoRv85ak8UrYkqd43NZpQJFQjn8rtgMfViBgAFaPDRh]
    -w, --websocket <websocket>                   [default: ws://127.0.0.1:1977]

ARGS:
    <transaction>    
```

If you cloned this repo, have [Rust installed](https://rustup.rs/) and initialized your nym client as shown above you
can run the following to transmit a hex-encoded Bitcoin `<transaction>` through a service provider at `<address>`:

```
cargo run --bin client -- -s <address> <transaction>
```

There is a default service provider at `7GAmWTUr3wude4LkRBJ78UmD2QMCgQvr8RCRJHW1fUYf.DHhHL8ZcnEEFq3UKuD7E31aWdnzuWdeJv1wRicj9n6tU@AmoRv85ak8UrYkqd43NZpQJFQjn8rtgMfViBgAFaPDRh`
which I run on a best-effort basis and which is chosen if the `-s` flag isn't provided. Please don't rely on it for anything critical.

If you want to transmit it to another `<network>` (supported networks: bitcoin, testnet, liquid), just specify the network
flag:

```
cargo run --bin client -- --network <network> -s <address> <transaction>
```

### BTC-BC Server
```
btcbc 0.1.0

USAGE:
    server [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -w, --websocket <websocket>     [default: ws://127.0.0.1:1977]
```

If you cloned this repo, have [Rust installed](https://rustup.rs/) and initialized your nym client as shown above you
can run the following to start the server:

```
cargo run --bin server -- --websocket ws://127.0.0.1:1978
```

It will output a log message telling you its nym address:

```
Feb 13 15:07:20.291  INFO server: Listening on 2DS4Cwf3x95A4KusRDSo9g8sJaJ4Z5xqNrftPprFHbkS.8Uj4NJjUeJE15bWE6K3aazXaaWbUDk28z5ZBo52GNKHm@DiYR9o8KgeQ81woKPYVAu4LNaAEg8SWkiufDCahNnPov
```

This address has to be given as an argument to the client when sending transactions.

## Debugging
If something isn't working as expected you can use the `RUST_LOG` environment variable to enable more verbose logging
(e.g. `RUST_LOG=debug`).