# Golemate - chess position evaluation on gWASM

![Continuous integration](https://github.com/golemfactory/golemate/workflows/Continuous%20integration/badge.svg)

## About Golemate
Golemate is a proof-of-concept of running chess engines under Golem. This repository consists of three parts:
* a common library which provides means to launch UCI-based engines
* a CLI
* a GUI

The library was written in mind to support multiple backends, currently implemented are:
* native: launches an engine compiled as a binary, directly on the host. This is the reference implementation.
* gWASM: launches an engine compiled to gWASM using Golem Clay. To get a gWASM-enabled engine,
visit https://github.com/marmistrz/FabChess/tree/gwasm-new


## Enable/disable features
Build of particular backends may be disabled using the cargo features for the library and the CLI.
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
* hash size: 1024 for native, 128 for gWASM (cf. #1)
* Golem client address: 127.0.0.1, port 61001

## Example use
Run a position analysis with the native backend
```
cargo run -- --engine /path/to/stockfish_20011801_x64_bmi2 --fen "r1bqkbnr/ppp2ppp/2n5/8/2BpP3/5N2/PP3PPP/RNBQK2R w KQkq - 1 6" --depth 20
```

Run a position analysis with the gWASM backend
```
cargo run -- --wasm $HOME/chess/FabChess/target/wasm32-unknown-emscripten/debug/uci_engine.wasm --js $HOME/chess/FabChess/target/wasm32-unknown-emscripten/debug/uci-engine.js --workspace $PWD/workspace --datadir $HOME/golem/datadir1/ --fen "8/4kp1p/1n2p3/1P6/8/8/p2rBPPP/R4K2 w - - 0 36" --depth 20
```

For more information about the available options, use `cargo run -- --help`. Note that the available switches depend on the enabled features.
