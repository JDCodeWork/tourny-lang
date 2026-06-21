use crate::{
    state::{Player, State},
    vm::StrPool,
};

use std::fmt::Write;

pub enum ShowTarget {
    Groups,
    Matches,
    Players,
}

impl State {
    pub fn render(&self, target: ShowTarget, str: &StrPool) -> String {
        let mut out = String::new();

        match target {
            ShowTarget::Groups => self.display_groups(&mut out, str),
            ShowTarget::Players => self.display_players(&mut out, str),
            ShowTarget::Matches => self.display_matches(&mut out, str),
        }

        out
    }

    fn display_groups(&self, out: &mut String, str: &StrPool) {
        let _ = writeln!(out, "\tGrupos\n======================\n");

        for (idx, group) in self.groups().enumerate() {
            let names = group
                .players()
                .map(|id| &str[self.player(*id).name])
                .collect::<Vec<_>>()
                .join("\n - ");

            let _ = writeln!(out, "Grupo {}\n - {}\n", idx + 1, names);
        }
    }

    fn display_matches(&self, out: &mut String, str: &StrPool) {
        let _ = writeln!(out, "\tEncuentros\n==========================\n");

        for (i, group) in self.groups().enumerate() {
            let _ = writeln!(out, "Grupo {}:", i + 1);

            for match_id in group.matches() {
                let match_ = self.match_(*match_id);

                let (p_id_1, p_id_2) = match_.players();
                let p_name_1 = &str[self.player(p_id_1).name];
                let p_name_2 = &str[self.player(p_id_2).name];

                let [r1, r2] = match_.result();

                let _ = writeln!(
                    out,
                    "[{}]      {p_name_1:<10} {r1} - {r2}\t{p_name_2}",
                    match_id.index()
                );
            }
            let _ = writeln!(out);
        }
    }

    pub fn display_players(&self, out: &mut String, str: &StrPool) {
        for Player { name } in self.players() {
            let name = &str[*name];

            let _ = writeln!(out, "{} |", name);
        }
    }
}
