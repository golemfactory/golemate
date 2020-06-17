use crate::backends::UciOutput;
use anyhow::Result;
use shakmaty::{fen::Fen, uci::Uci, Color, Move};
use std::convert::TryInto;

pub enum Advantage {
    Centipawns(u32),
    /// mate in # of moves
    Mate(u32),
    Equality,
}

pub struct AnalysisResult {
    pub advantage_side: Color,
    pub advantage: Advantage,
    pub depth: u32,
    pub pv: Vec<Uci>,
    pub best_move: Move,
}

impl AnalysisResult {
    pub fn describe_advantage(&self) -> String {
        use Advantage::*;
        match self.advantage {
            Equality => "The position is equal.".to_owned(),
            Mate(moves) => format!("{:?} has a mate in {} moves", self.advantage_side, moves),
            Centipawns(cp) => format!("{:?} has {} centipawns advantage", self.advantage_side, cp),
        }
    }

    pub fn describe(&self) -> String {
        format!(
            "Analysis depth: {}.\n{}.\nThe best move is {}.",
            self.depth,
            self.describe_advantage(),
            self.best_move
        )
    }
}

fn other_color(color: Color) -> Color {
    use Color::*;
    match color {
        White => Black,
        Black => White,
    }
}

// TODO create a proper parser, this is too hacky.
// There's a lot of expects here. All of them are protocol violations
// and will be removed once we have a proper parser.
pub fn interpret_uci(startpos_fen: Fen, uci: UciOutput) -> Result<AnalysisResult> {
    let our_side = startpos_fen.turn;
    let position: shakmaty::Chess = startpos_fen.position()?;

    let mut pv = Vec::new();
    let mut depth = 0;
    let mut advantage = Advantage::Mate(0);
    let mut advantage_side = Color::White;
    let mut best_move = None;
    for line in uci {
        if line.is_empty() {
            continue;
        }
        let mut words = line.split_whitespace();
        let cmd = words.next().expect("line should be non-empty");
        match cmd {
            "info" => {
                loop {
                    let cmdtype = words.next().expect("type should be non-empty");
                    match cmdtype {
                        "String" => break,
                        "depth" => {
                            depth = words
                                .next()
                                .expect("argument missing")
                                .parse()
                                .expect("parse error")
                        }
                        "pv" => {
                            pv = words
                                .map(|w| w.parse().expect("UCI move parse error"))
                                .collect();
                            break;
                        } //
                        "score" => {
                            let sctype = words.next().expect("argument missing");
                            let scval: i32 = words
                                .next()
                                .expect("argument missing")
                                .parse()
                                .expect("parse error");
                            advantage_side = if scval < 0 {
                                other_color(our_side.clone())
                            } else {
                                our_side.clone()
                            };
                            let adv_val = scval.abs().try_into()?;
                            advantage = match sctype {
                                _ if scval == 0 => Advantage::Equality,
                                "cp" => Advantage::Centipawns(adv_val),
                                "mate" => Advantage::Mate(adv_val),
                                _ => panic!("unexpected score mode"),
                            };
                        }
                        _ => {
                            words.next().expect("the skipped arg should be non-empty");
                        }
                    }
                }
            }
            "bestmove" => {
                let bmove = words.next().expect("argument missing");
                let bmove: Uci = bmove.parse().expect("UCI move parse error");
                let bmove = bmove
                    .to_move(&position)
                    .expect("engine returned an illegalm move");
                best_move = Some(bmove)
            }
            _ => {}
        }
    }

    Ok(AnalysisResult {
        advantage,
        advantage_side,
        depth,
        pv,
        best_move: best_move.expect("best move not set"),
    })
}
