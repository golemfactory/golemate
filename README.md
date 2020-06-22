# Golemate - chess position evaluation on gWASM

![Continuous integration](https://github.com/golemfactory/golemate/workflows/Continuous%20integration/badge.svg)

## About Golemate
Golemate is a proof-of-concept of running chess engines under Golem. At the moment, it searches for the best move for a given position. We would like to have human vs Golem matches in the future. This repository consists of three parts:
* a common library which provides means to launch engines compliant with the [UCI protocol]
* a CLI client application
* a GUI client application

The library was written in mind to support multiple backends, currently implemented are:
### Native backend
Launches a UCI-compatible engine compiled as a binary, directly on the host. This is the reference implementation.

### gWASM backend
Launches a UCI-compatible engine compiled to gWASM, using Golem Clay. Running jobs requires a working instance of Golem in either mainnet or testnet mode.

For more information how to setup Golem, consult the [Golem docs](https://docs.golem.network).
To get a gWASM-compatible engine,
you may follow the build instructions of [our fork of FabChess](https://github.com/golemfactory/FabChess). You can try another engine if it is [compatible with gWASM](https://docs.golem.network/#/Products/gWASM/Sandboxing).

## Enable/disable features
Build of particular backends may be disabled using the cargo features for the library and the CLI. By default all backends are enabled.

For instance, to disable the native backend and build only the gWASM backend, use
```
cargo build --features gwasm --no-default-features
```
or disable the gWASM backend and build only the native backend, use
```
cargo build --features native --no-default-features
```

Note that the GUI doesn't currently support feature-gating described above.

## Hardcoded parameters
Since this is a PoC, not all parameters are configurable. In particular, the following are hardcoded:
* hash size: 1024 for native, 128 for gWASM (cf. [this issue](https://github.com/golemfactory/FabChess/issues/1))
* Golem client address: 127.0.0.1, port 61001

## Example use
You can transcode a chess position into `fen` format [here](https://lichess.org/editor), for instance. 

### CLI
Run a position analysis with the native backend
```
cargo run -- --engine /path/to/stockfish_20011801_x64_bmi2 --fen "r1bqkbnr/ppp2ppp/2n5/8/2BpP3/5N2/PP3PPP/RNBQK2R w KQkq - 1 6" --depth 20
```

Run a position analysis with the gWASM backend
```
cargo run -- --wasm /path/to/uci_engine.wasm --js /path/to/uci-engine.js --workspace workspace --datadir /path/to/golem/datadir1/ --fen "8/4kp1p/1n2p3/1P6/8/8/p2rBPPP/R4K2 w - - 0 36" --depth 20
```

For more information about the available options, use `cargo run -- --help`. Note that their availability may depend on the enabled features.

### GUI
To launch the GUI, run
```
cargo run -p golemate-gui
```
The GUI requires Gtk+ 3.16 or newer.

[UCI protocol]: http://wbec-ridderkerk.nl/html/UCIProtocol.html
