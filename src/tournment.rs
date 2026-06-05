use rand::seq::SliceRandom;
use std::{slice, vec};

use crate::vm::StrRef;

#[derive(Debug)]
pub struct PlayerId(pub u8);

#[derive(Debug)]
pub struct Player {
    pub name: StrRef,
}

impl From<u8> for Player {
    fn from(value: u8) -> Self {
        Self {
            name: StrRef(value),
        }
    }
}

#[derive(Debug)]
pub struct Group {
    players: Vec<PlayerId>,
    round: u8,
}

impl Group {
    pub fn iter_players(&self) -> slice::Iter<'_, PlayerId> {
        self.players.iter()
    }
}

#[derive(Default, Debug)]
pub struct State {
    players: Vec<Player>,
    groups: Vec<Group>,
    curr_round: u8,
}

impl State {
    pub fn add_player(&mut self, name: StrRef) {
        self.players.push(Player { name });
    }

    pub fn get_player(&self, id: &PlayerId) -> &Player {
        &self.players[id.0 as usize]
    }

    pub fn iter_players(&self) -> slice::Iter<'_, Player> {
        self.players.iter()
    }

    pub fn iter_groups(&self) -> slice::Iter<'_, Group> {
        self.groups.iter()
    }

    pub fn make_groups(&mut self) {
        let mut players_ids: Vec<PlayerId> =
            (0..self.players.len()).map(|i| PlayerId(i as u8)).collect();

        let mut rng = rand::rng();
        players_ids.shuffle(&mut rng);

        let max_players_per_goup = 4;
        let groups_count = players_ids.len().div_ceil(max_players_per_goup);

        let mut groups: Vec<Group> = (0..groups_count)
            .map(|_| Group {
                players: vec![],
                round: self.curr_round,
            })
            .collect();

        for (i, player_id) in players_ids.into_iter().enumerate() {
            groups[i % groups_count].players.push(player_id);
        }

        self.groups.extend(groups);
    }
}
