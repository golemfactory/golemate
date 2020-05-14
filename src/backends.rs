use anyhow::{Context, Result};
use std::path::PathBuf;

pub struct UciOption<'a> {
    name: &'a str,
    value: u32,
}

impl UciOption<'_> {
    fn uci_set_msg(&self) -> String {
        format!("setoption name {} value {}", self.name, self.value)
    }
}

pub trait UciBackend {
    fn execute_uci(&self, uci: Vec<String>) -> Result<()>;
    fn get_uci_opts(&self) -> Vec<UciOption<'static>>;

    fn generate_uci(&self, fen: &str, depth: u32) -> Vec<String> {
        let intro = vec!["uci".to_owned()];
        let outro = vec!["ucinewgame".to_owned(), "quit".to_owned()];
        let mut cmds = intro;
        cmds.extend(self.get_uci_opts().iter().map(UciOption::uci_set_msg));
        cmds.push(format!("position fen {}", fen));
        cmds.push(format!("go depth {}", depth));
        cmds.extend(outro);
        cmds
    }
}

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

pub struct GWasmUci {}

impl GWasmUci {
    pub fn new() -> Self {
        Self {}
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
        unimplemented!("gWASM backend")
    }
}
