use super::{UciBackend, UciOption};
use anyhow::{Context, Result};
use std::path::PathBuf;

/// Runs a client locally
pub struct NativeUci {
    engine_path: PathBuf,
}

impl NativeUci {
    pub fn new(engine_path: PathBuf) -> Self {
        Self { engine_path }
    }
}

impl UciBackend for NativeUci {
    fn get_uci_opts(&self) -> Vec<UciOption<'static>> {
        // TODO detect/set values
        vec![
            UciOption {
                name: "Threads",
                value: 8,
            },
            UciOption {
                name: "Hash",
                value: 1024,
            },
        ]
    }

    fn execute_uci(&self, uci: Vec<String>) -> Result<()> {
        use std::io::{LineWriter, Write};
        use std::process::{Command, Stdio};

        let mut child = Command::new(&self.engine_path)
            .stdin(Stdio::piped())
            .spawn()
            .context("running the UCI engine")?;

        {
            let stdin = child.stdin.as_mut().context("opening stdin")?;
            let mut stdin = LineWriter::new(stdin);
            for line in uci {
                println!("{}", line);
                writeln!(stdin, "{}", line)?;
            }
        }

        child.wait().context("waiting for the child process")?;
        Ok(())
    }
}
