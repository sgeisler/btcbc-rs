# Nym Bitcoin Broadcaster

This is a rewrite of my [bitcoin transaction broadcaster hack](https://github.com/sgeisler/nym_btc_broadcast) in the
hope to make it somewhat robust to be able to run it as an unattended service, something I don't really trust (my)
python code with.

To use it you have to initialize and run a [Nym client](https://nymtech.net/docs/build-peapps/native-client/):
```
nym-client init --id test
nym-client run --id test
```

Then you can run the bitcoin transaction broadcasting service by just pointing it to the websocket opened by the Nym
client:
```
cargo run -- ws://127.0.0.1:1977
```

Shortly after starting it will output a string like this:
```
Ok(Text("{\"type\":\"selfAddress\",\"address\":\"H5FfVbeSPkmCrxQ1UBSRmsUEWrogc1NoBd1VjrQ3x7FM@CuJWcKgY7ktjYrszJck2sqZoPMrc9U1BGK8Wjtrh853v\"}"))
```

telling you that the service's "address" is:
```
H5FfVbeSPkmCrxQ1UBSRmsUEWrogc1NoBd1VjrQ3x7FM@CuJWcKgY7ktjYrszJck2sqZoPMrc9U1BGK8Wjtrh853v
```

Once it's running you can send messages in the form of `<hex-tx>` (Bitcoin), `testnet:<hex-tx>` (Bitcoin Testnet) and
 `liquid:<hex-tx>` (Liquid) to it and it will broadcast them via the [blockstream.info](https://blockstream.info/) API.