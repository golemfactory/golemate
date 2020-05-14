use std::path::PathBuf;
use structopt::StructOpt;

fn is_fen(s: String) -> Result<(), String> {
    use fen::BoardState;

    BoardState::from_fen(&s)
        .map(|_| ())
        .map_err(|e| format!("{:?}", e))
    // TODO add proper error printing to fen
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "golemate",
    author = "Marcin Mielniczuk <marmistrz.dev@zoho.eu>",
    about = "Chess position solver using gWASM"
)]
pub(crate) struct Opts {
    #[structopt(short = "f", long = "fen", validator = is_fen)]
    pub fen: String,
    #[structopt(short = "d", long = "depth")]
    pub depth: u32,
    #[structopt(
        short = "e",
        long = "engine",
        help = "Path to a local engine to be used instead of gWASM"
    )]
    pub engine: Option<PathBuf>,
}
