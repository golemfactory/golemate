use super::{UciBackend, UciInput, UciOption, UciOutput};
use anyhow::{anyhow, Context, Result};
use gwasm_api::prelude::*;
use std::path::{Path, PathBuf};
use std::{fs, io};

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

    fn execute_uci(&self, uci: UciInput) -> Result<UciOutput> {
        let binary = GWasmBinary {
            js: &self.js,
            wasm: &self.wasm,
        };

        let mut uci = uci.join("\n");
        uci.push_str("\n");
        let input = uci.as_bytes();

        fs::create_dir(&self.workspace)
            .map_err(|e| match e.kind() {
                io::ErrorKind::AlreadyExists => anyhow!("Workspace already exists"),
                _ => e.into(),
            })
            .context("creating the workspace")?;

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

        single_subtask_get_output(computed_task)
    }
}

fn single_subtask_get_output(task: gwasm_api::task::ComputedTask) -> Result<UciOutput> {
    use std::io::BufRead;
    use std::io::Result as IoResult;

    let mut subtask = task.subtasks;
    assert_eq!(subtask.len(), 1);
    let mut outputs = subtask[0].data.values_mut();
    let output = outputs.next().expect("subtask values should be non-empty");
    //outputs.next().expect_none("too many subtask values");
    let res: IoResult<Vec<_>> = output.lines().collect();
    res.map_err(Into::into)
}

struct ProgressTracker;

impl ProgressUpdate for ProgressTracker {
    fn update(&self, progress: f64) {
        println!("Current progress = {}", progress);
    }
}
