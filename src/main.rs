mod analysis;
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
    let backend: Box<dyn UciBackend> = opts.backend()?;
    let cmds = backend.generate_uci(&opts.fen.to_string(), opts.depth);
    let output = backend.execute_uci(cmds).context("Executing UCI")?;
    if opts.raw_uci {
        for line in output {
            println!("{}", line);
        }
    } else {
        let an_res = analysis::interpret_uci(opts.fen, output)?;
        println!(
            "Analysis depth: {}. {}. The best move is {}",
            an_res.depth,
            an_res.describe_advantage(),
            an_res.best_move
        )
    }

    Ok(())
}
