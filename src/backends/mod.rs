mod gwasm;
mod native;

pub use gwasm::*;
pub use native::*;

use anyhow::Result;

pub struct UciOption<'a> {
    name: &'a str,
    value: u32,
}

impl UciOption<'_> {
    fn uci_set_msg(&self) -> String {
        format!("setoption name {} value {}", self.name, self.value)
    }
}

pub trait UciBackend {
    fn execute_uci(&self, uci: Vec<String>) -> Result<()>;
    fn get_uci_opts(&self) -> Vec<UciOption<'static>>;

    fn generate_uci(&self, fen: &str, depth: u32) -> Vec<String> {
        let intro = vec!["uci".to_owned()];
        let outro = vec!["ucinewgame".to_owned(), "quit".to_owned()];
        let mut cmds = intro;
        cmds.extend(self.get_uci_opts().iter().map(UciOption::uci_set_msg));
        cmds.push(format!("position fen {}", fen));
        cmds.push(format!("go depth {}", depth));
        cmds.extend(outro);
        cmds
    }
}
