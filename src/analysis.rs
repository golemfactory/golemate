use crate::backends::UciOutput;
use shakmaty::{fen::Fen, Color};

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
    pub pv: Vec<String>,
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
}

fn other_color(color: Color) -> Color {
    use Color::*;
    match color {
        White => Black,
        Black => White,
    }
}

// TODO create a proper parser, this is too hacky
pub fn interpret_uci(startpos_fen: Fen, uci: UciOutput) -> AnalysisResult {
    let our_side = startpos_fen.turn;

    let mut pv = Vec::new();
    let mut depth = 0;
    let mut advantage = Advantage::Mate(0);
    let mut advantage_side = Color::White;
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
                        "String" => continue,
                        "depth" => {
                            depth = words
                                .next()
                                .expect("argument missing")
                                .parse()
                                .expect("parse error")
                        }
                        "pv" => {
                            pv = words.map(|w| w.to_owned()).collect();
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
                            let adv_val = scval.abs() as u32;
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
            _ => {}
        }
    }

    AnalysisResult {
        advantage,
        advantage_side,
        depth,
        pv,
    }
}
