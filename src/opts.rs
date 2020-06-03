use crate::{backends, UciBackend};
use anyhow::Result;
use shakmaty::fen::Fen;
use std::path::PathBuf;
use structopt::{clap::ArgGroup, StructOpt};

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
    #[structopt(short, long)]
    pub fen: Fen,

    #[structopt(short, long, help = "search depth")]
    pub depth: u32,

    #[structopt(short, long = "raw", help = "output raw UCI instead of analysis")]
    pub raw_uci: bool,

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
    pub(crate) fn backend(&self) -> Result<Box<dyn UciBackend>> {
        #[cfg(feature = "gwasm")]
        {
            let opt = &self.gwasm_opts;
            if opt.wasm_path.is_some() {
                let backend = backends::GWasmUci::new(
                    &opt.wasm_path.clone().unwrap(),
                    &opt.js_path.clone().unwrap(),
                    opt.workspace.clone().unwrap(),
                    opt.datadir.clone().unwrap(),
                )?;
                return Ok(Box::new(backend));
            }
        }
        #[cfg(feature = "native")]
        {
            if self.engine.is_some() {
                let backend = backends::NativeUci::new(self.engine.clone().unwrap());
                return Ok(Box::new(backend));
            }
        }
        unreachable!("Internal error: command-line was not properly verified");
    }
}
