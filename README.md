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