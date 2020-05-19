mod backends;
mod opts;

use structopt::StructOpt;

use anyhow::{Context, Result};
use backends::UciBackend;
use opts::Opts;

fn main() -> Result<()> {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    let opts = Opts::from_args();
    let backend: Box<dyn UciBackend> = if let Some(path) = opts.engine {
        Box::new(backends::NativeUci::new(path))
    } else {
        // FIXME unwrap
        Box::new(backends::GWasmUci::new(
            &opts.wasm_path.unwrap(),
            &opts.js_path.unwrap(),
        )?)
    };
    let cmds = backend.generate_uci(&opts.fen, opts.depth);
    backend.execute_uci(cmds).context("Executing UCI")?;
    Ok(())
}
