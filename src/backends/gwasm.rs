use super::{UciBackend, UciOption};
use anyhow::{Context, Result};
use gwasm_api::prelude::*;
use std::path::{Path, PathBuf};

pub struct GWasmUci {
    wasm: Vec<u8>,
    js: Vec<u8>,
    workspace: PathBuf,
    datadir: PathBuf,
}

impl GWasmUci {
    pub fn new(
        wasm_path: &Path,
        js_path: &Path,
        workspace: PathBuf,
        datadir: PathBuf,
    ) -> Result<Self> {
        use std::fs::read;
        let wasm = read(wasm_path).context("reading the engine WASM")?;
        let js = read(js_path).context("reading the engine JS")?;
        Ok(Self {
            wasm,
            js,
            workspace,
            datadir,
        })
    }
}

impl UciBackend for GWasmUci {
    fn get_uci_opts(&self) -> Vec<UciOption<'static>> {
        // TODO detect based on golem info
        vec![UciOption {
            name: "Hash",
            value: 128,
        }]
    }

    fn execute_uci(&self, uci: Vec<String>) -> Result<()> {
        let binary = GWasmBinary {
            js: &self.js,
            wasm: &self.wasm,
        };

        let mut uci = uci.join("\n");
        uci.push_str("\n");
        let input = uci.as_bytes();

        // TODO create or check the workspace

        let task = TaskBuilder::try_new(&self.workspace, binary)?
            .name("golemate")
            .push_subtask_data(input)
            .build()
            .context("building the gwasm task")?;

        // FIXME address, port
        let computed_task = compute(
            self.datadir.clone(),
            "127.0.0.1",
            61001,
            Net::TestNet,
            task,
            ProgressTracker,
        )
        .context("computing the gwasm task")?;

        println!("done: {:?}", computed_task);
        Ok(())
    }
}

struct ProgressTracker;

impl ProgressUpdate for ProgressTracker {
    fn update(&self, progress: f64) {
        println!("Current progress = {}", progress);
    }
}
