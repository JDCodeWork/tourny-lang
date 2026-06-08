use crate::state::{Player, State};

use std::fmt::Write;

impl State {
    pub fn display_groups(&self, strings: &[String]) -> String {
        let mut out = String::new();
        for (idx, group) in self.iter_groups().enumerate() {
            let names = group
                .iter_players()
                .map(|id| strings[self.get_player(id).name.0 as usize].as_str())
                .collect::<Vec<_>>()
                .join(", ");

            // TODO
            writeln!(out, "Group {}: {}", idx + 1, names).unwrap();
        }

        out
    }

    pub fn display_players(&self, strings: &[String]) -> String {
        let mut out = String::new();

        for Player { name } in self.iter_players() {
            let name = &strings[name.0 as usize];

            out += &format!(" {name} | ");
        }
        out += "\n";

        out
    }
}
