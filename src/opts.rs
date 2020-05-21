use crate::{backends, UciBackend};
use anyhow::Result;
use std::path::PathBuf;
use structopt::{clap::ArgGroup, StructOpt};

fn is_fen(s: String) -> Result<(), String> {
    use fen::BoardState;

    BoardState::from_fen(&s)
        .map(|_| ())
        .map_err(|e| format!("{:?}", e))
    // TODO add proper error printing to fen
}

#[cfg(not(any(feature = "gwasm", feature = "native")))]
compile_error!("At least one backend must be enabled");

#[derive(Debug, StructOpt)]
#[structopt(
    name = "golemate",
    author = "Marcin Mielniczuk <marmistrz.dev@zoho.eu>",
    about = "Chess position solver using gWASM"
)]
#[structopt(group = ArgGroup::with_name("binary").required(true))]
pub(crate) struct Opts {
    #[structopt(short, long, validator = is_fen)]
    pub fen: String,
    #[structopt(short, long, help = "search depth")]
    pub depth: u32,
    #[cfg(feature = "native")]
    #[structopt(
        short = "e",
        long = "engine",
        help = "path to a local engine to be used instead of gWASM",
        group = "binary"
    )]
    pub engine: Option<PathBuf>,
    #[cfg(feature = "gwasm")]
    #[structopt(
        short,
        long = "wasm",
        help = "path to the WASM part of the gWASM binary",
        group = "binary",
        requires = "js-path",
        requires = "workspace",
        requires = "datadir"
    )]
    pub wasm_path: Option<PathBuf>,
    #[cfg(feature = "gwasm")]
    #[structopt(short, long = "js", help = "path to the JS part of the gWASM binary")]
    pub js_path: Option<PathBuf>,
    #[cfg(feature = "gwasm")]
    #[structopt(long)]
    pub workspace: Option<PathBuf>,
    #[cfg(feature = "gwasm")]
    #[structopt(long)]
    pub datadir: Option<PathBuf>,
}

impl Opts {
    pub(crate) fn into_backend(self) -> Result<Box<dyn UciBackend>> {
        #[cfg(feature = "gwasm")]
        {
            if self.wasm_path.is_some() {
                let backend = backends::GWasmUci::new(
                    &self.wasm_path.unwrap(),
                    &self.js_path.unwrap(),
                    self.workspace.unwrap(),
                    self.datadir.unwrap(),
                )?;
                return Ok(Box::new(backend));
            }
        }
        #[cfg(feature = "native")]
        {
            if self.engine.is_some() {
                let backend = backends::NativeUci::new(self.engine.unwrap());
                return Ok(Box::new(backend));
            }
        }
        unreachable!("Internal error: command-line was not properly verified");
    }
}
