use crate::{
    state::{Player, State},
    vm::StrPool,
};

use std::fmt::Write;

// TODO: Create Show enum with the display target and implement render funtion inside of state
impl State {
    pub fn display_groups(&self, str: &StrPool) -> String {
        let mut out = String::new();
        for (idx, group) in self.groups().enumerate() {
            let names = group
                .players()
                .map(|id| &str[self[*id].name])
                .collect::<Vec<_>>()
                .join(", ");

            // TODO: unwrap
            writeln!(out, "Group {}: {}", idx + 1, names).unwrap();
        }

        out
    }

    pub fn display_players(&self, str: &StrPool) -> String {
        let mut out = String::new();

        for Player { name } in self.players() {
            let name = &str[*name];

            out += &format!(" {name} | ");
        }
        out += "\n";

        out
    }
}
