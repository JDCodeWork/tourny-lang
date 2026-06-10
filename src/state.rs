use rand::seq::SliceRandom;
use std::{ops::Index, slice, vec};

use crate::vm::StrId;

#[derive(Debug, Clone, Copy)]
pub struct PlayerId(u8);

impl PlayerId {
    pub fn index(&self) -> usize {
        self.0 as usize
    }
}

#[derive(Debug)]
pub struct Player {
    pub name: StrId,
}

impl From<u8> for Player {
    fn from(value: u8) -> Self {
        Self {
            name: StrId::new(value),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GroupId(u8);
impl GroupId {
    pub fn index(&self) -> usize {
        self.0 as usize
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Group {
    players: Vec<PlayerId>,
    round: u8,
}

impl Group {
    pub fn players(&self) -> slice::Iter<'_, PlayerId> {
        self.players.iter()
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Match {
    group: GroupId,
    fst_player: PlayerId,
    snd_player: PlayerId,
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
    pub fn add_player(&mut self, name: StrId) {
        self.players.push(Player { name });
    }

    pub fn matches(&self) -> slice::Iter<'_, Match> {
        self.matches.iter()
    }

    pub fn players(&self) -> slice::Iter<'_, Player> {
        self.players.iter()
    }

    pub fn groups(&self) -> slice::Iter<'_, Group> {
        self.groups.iter()
    }

    pub fn make_groups(&mut self) {
        let mut players_ids: Vec<PlayerId> =
            (0..self.players.len()).map(|i| PlayerId(i as u8)).collect();

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
        let groups: Vec<GroupId> = (0..self.groups.len()).map(|i| GroupId(i as u8)).collect();

        for group in groups {
            self.make_matches_group(group);
        }
    }

    fn make_matches_group(&mut self, group: GroupId) {
        let mut players = self[group].players.clone();

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

impl Index<PlayerId> for State {
    type Output = Player;

    fn index(&self, id: PlayerId) -> &Self::Output {
        &self.players[id.index()]
    }
}

impl Index<GroupId> for State {
    type Output = Group;

    fn index(&self, id: GroupId) -> &Self::Output {
        &self.groups[id.index()]
    }
}
