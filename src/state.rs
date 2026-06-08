use rand::seq::SliceRandom;
use std::{slice, vec};

use crate::vm::StrRef;

#[derive(Debug, Clone, Copy)]
pub struct PlayerRef(pub u8);

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

#[derive(Debug, Clone, Copy)]
struct GroupRef(u8);

#[derive(Debug)]
pub struct Group {
    players: Vec<PlayerRef>,
    round: u8,
}

impl Group {
    pub fn iter_players(&self) -> slice::Iter<'_, PlayerRef> {
        self.players.iter()
    }
}

#[derive(Debug)]
pub struct Match {
    group: GroupRef,
    fst_player: PlayerRef,
    snd_player: PlayerRef,
    result: [u8; 2],
}

#[derive(Default, Debug)]
pub struct State {
    players: Vec<Player>,
    groups: Vec<Group>,
    matches: Vec<Match>,
    curr_round: u8,
    curr_group_count: u8,
}

impl State {
    pub fn add_player(&mut self, name: StrRef) {
        self.players.push(Player { name });
    }

    pub fn get_player(&self, id: &PlayerRef) -> &Player {
        &self.players[id.0 as usize]
    }

    pub fn get_group(&self, id: &GroupRef) -> &Group {
        &self.groups[id.0 as usize]
    }

    pub fn iter_matches(&self) -> slice::Iter<'_, Match> {
        self.matches.iter()
    }

    pub fn iter_players(&self) -> slice::Iter<'_, Player> {
        self.players.iter()
    }

    pub fn iter_groups(&self) -> slice::Iter<'_, Group> {
        self.groups.iter()
    }

    pub fn make_groups(&mut self) {
        let mut players_ids: Vec<PlayerRef> = (0..self.players.len())
            .map(|i| PlayerRef(i as u8))
            .collect();

        let mut rng = rand::rng();
        players_ids.shuffle(&mut rng);

        let max_players_per_goup = 4;
        let groups_count = players_ids.len().div_ceil(max_players_per_goup);

        self.curr_group_count = groups_count as u8;

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

    pub fn make_matches(&mut self) {
        let groups: Vec<GroupRef> = (0..self.groups.len()).map(|i| GroupRef(i as u8)).collect();

        for group in groups {
            self.make_matches_group(group);
        }
    }

    fn make_matches_group(&mut self, group: GroupRef) {
        let mut players = self.groups[group.0 as usize].players.clone();

        let rounds = players.len() - 1;
        let matches_per_round = players.len() / 2;

        for _round in 0..rounds {
            for i in 0..matches_per_round {
                let match_ = Match {
                    fst_player: players[i],
                    snd_player: players[players.len() - 1 - i],
                    result: [0, 0],
                    group,
                };

                self.matches.push(match_);
            }

            let last = players.pop().unwrap();
            players.insert(1, last);
        }
    }
}
