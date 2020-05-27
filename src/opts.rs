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
#[cfg(feature = "gwasm")]
pub(crate) struct GWasmOpts {
    #[cfg(feature = "gwasm")]
    #[structopt(
        short,
        long = "wasm",
        help = "path to the WASM part of the gWASM binary",
        group = "backend",
        requires = "js-path",
        requires = "workspace",
        requires = "datadir"
    )]
    pub wasm_path: Option<PathBuf>,

    #[structopt(short, long = "js", help = "path to the JS part of the gWASM binary")]
    pub js_path: Option<PathBuf>,

    #[structopt(long)]
    pub workspace: Option<PathBuf>,

    #[structopt(long)]
    pub datadir: Option<PathBuf>,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "golemate",
    author = "Marcin Mielniczuk <marmistrz.dev@zoho.eu>",
    about = "Chess position solver using gWASM"
)]
#[structopt(group = ArgGroup::with_name("backend").required(true))]
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
        group = "backend"
    )]
    pub engine: Option<PathBuf>,

    #[cfg(feature = "gwasm")]
    #[structopt(flatten)]
    pub gwasm_opts: GWasmOpts,
}

impl Opts {
    pub(crate) fn into_backend(self) -> Result<Box<dyn UciBackend>> {
        #[cfg(feature = "gwasm")]
        {
            let opt = self.gwasm_opts;
            if opt.wasm_path.is_some() {
                let backend = backends::GWasmUci::new(
                    &opt.wasm_path.unwrap(),
                    &opt.js_path.unwrap(),
                    opt.workspace.unwrap(),
                    opt.datadir.unwrap(),
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
