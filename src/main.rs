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
    let fen = opts.fen.clone();
    let depth = opts.depth;
    let backend: Box<dyn UciBackend> = opts.into_backend()?;
    let cmds = backend.generate_uci(&fen, depth);
    let output = backend.execute_uci(cmds).context("Executing UCI")?;
    println!("Task finished, UCI output:");
    for line in output {
        println!("{}", line);
    }
    Ok(())
}
