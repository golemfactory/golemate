use anyhow::{Context, Result};
use golemate::{analysis, backends, backends::UciBackend};
use shakmaty::fen::Fen;
use std::path::PathBuf;
use structopt::{clap::ArgGroup, StructOpt};

#[cfg(not(any(feature = "gwasm", feature = "native")))]
compile_error!("At least one backend must be enabled");

#[derive(Debug, StructOpt)]
#[cfg(feature = "gwasm")]
pub struct GWasmOpts {
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

    #[structopt(long, help = "path to the workspace used by Golem")]
    pub workspace: Option<PathBuf>,

    #[structopt(long, help = "path to the Golem client data directory")]
    pub datadir: Option<PathBuf>,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "golemate",
    author = "Marcin Mielniczuk <marmistrz.dev@zoho.eu>",
    about = "Chess position solver using gWASM"
)]
#[structopt(group = ArgGroup::with_name("backend").required(true))]
pub struct Opts {
    #[structopt(short, long, help = "position in the FEN format")]
    pub fen: Fen,

    #[structopt(short, long, help = "analysis depth")]
    pub depth: u32,

    #[structopt(short, long = "raw", help = "output raw UCI instead of analysis")]
    pub raw_uci: bool,

    #[cfg(feature = "native")]
    #[structopt(
        short = "e",
        long = "engine",
        help = "path to a local engine to be used by the native backend",
        group = "backend"
    )]
    pub engine: Option<PathBuf>,

    #[cfg(feature = "gwasm")]
    #[structopt(flatten)]
    pub gwasm_opts: GWasmOpts,
}

impl Opts {
    pub fn backend(&self) -> Result<Box<dyn UciBackend>> {
        #[cfg(feature = "gwasm")]
        {
            let opt = &self.gwasm_opts;
            if opt.wasm_path.is_some() {
                let backend = backends::GWasmUci::new(
                    &opt.wasm_path.clone().expect("inconsistent wasm opts"),
                    &opt.js_path.clone().expect("inconsistent wasm opts"),
                    opt.workspace.clone().expect("inconsistent wasm opts"),
                    opt.datadir.clone().expect("inconsistent wasm opts"),
                )?;
                return Ok(Box::new(backend));
            }
        }
        #[cfg(feature = "native")]
        {
            if let Some(engine) = &self.engine {
                let backend = backends::NativeUci::new(engine.to_owned());
                return Ok(Box::new(backend));
            }
        }
        unreachable!("Internal error: command-line was not properly verified");
    }
}

fn main() -> Result<()> {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    let opts = Opts::from_args();
    let backend: Box<dyn UciBackend> = opts.backend()?;
    let cmds = backend.generate_uci(&opts.fen.to_string(), opts.depth);
    let output = backend.execute_uci(cmds).context("Executing UCI")?;
    if opts.raw_uci {
        for line in output {
            println!("{}", line);
        }
    } else {
        let an_res = analysis::interpret_uci(opts.fen, output)?;
        println!("{}", an_res.describe());
    }

    Ok(())
}
