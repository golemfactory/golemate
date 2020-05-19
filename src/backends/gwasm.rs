use super::{UciBackend, UciOption};
use anyhow::{Context, Result};
use std::path::Path;

pub struct GWasmUci {
    wasm: Vec<u8>,
    js: Vec<u8>,
}

impl GWasmUci {
    pub fn new(wasm_path: &Path, js_path: &Path) -> Result<Self> {
        use std::fs::read;
        let wasm = read(wasm_path).context("reading the engine WASM")?;
        let js = read(js_path).context("reading the engine JS")?;
        Ok(Self { wasm, js })
    }
}

impl UciBackend for GWasmUci {
    fn get_uci_opts(&self) -> Vec<UciOption<'static>> {
        // TODO detect based on golem info
        vec![UciOption {
            name: "Hash",
            value: 1024,
        }]
    }

    fn execute_uci(&self, _uci: Vec<String>) -> Result<()> {
        use gwasm_api::prelude::*;
        let _binary = GWasmBinary {
            js: &self.js,
            wasm: &self.wasm,
        };
        unimplemented!("gWASM backend")
    }
}
