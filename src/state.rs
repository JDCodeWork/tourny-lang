use rand::seq::SliceRandom;
use std::{slice, vec};

use crate::vm::StrId;

macro_rules! define_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name(u8);

        impl $name {
            pub fn new(id: u8) -> Self {
                Self(id)
            }

            pub fn index(self) -> usize {
                self.0 as usize
            }
        }
    };
}

define_id!(PlayerId);
#[derive(Debug)]
pub struct Player {
    pub name: StrId,
}

define_id!(GroupId);
#[derive(Debug)]
#[allow(dead_code)]
pub struct Group {
    players: Vec<PlayerId>,
    matches: Vec<MatchId>,
    round: u8,
}

impl Group {
    pub fn players(&self) -> slice::Iter<'_, PlayerId> {
        self.players.iter()
    }

    pub fn matches(&self) -> slice::Iter<'_, MatchId> {
        self.matches.iter()
    }
}

define_id!(MatchId);
#[derive(Debug)]
#[allow(dead_code)]
pub struct Match {
    fst_player: PlayerId,
    snd_player: PlayerId,
    result: [u8; 2],
}

impl Match {
    pub fn players(&self) -> (PlayerId, PlayerId) {
        (self.fst_player, self.snd_player)
    }

    pub fn result(&self) -> [u8; 2] {
        self.result
    }
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
                matches: vec![],
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

        for group_id in groups {
            let matches = self.make_matches_group(group_id);

            let group = &mut self.groups[group_id.index()];
            group.matches.extend(matches);
        }
    }

    fn make_matches_group(&mut self, id: GroupId) -> Vec<MatchId> {
        let players = self.group(id).players.clone();
        let mut matches_ids = Vec::new();

        for i in 0..players.len() {
            for j in (i + 1)..players.len() {
                let match_ = Match {
                    fst_player: players[i],
                    snd_player: players[j],
                    result: [0, 0],
                };
                let match_id = MatchId::new(self.matches.len() as u8);

                matches_ids.push(match_id);
                self.matches.push(match_);
            }
        }

        matches_ids
    }

    pub fn player(&self, id: PlayerId) -> &Player {
        &self.players[id.index()]
    }

    pub fn match_(&self, id: MatchId) -> &Match {
        &self.matches[id.index()]
    }

    pub fn group(&self, id: GroupId) -> &Group {
        &self.groups[id.index()]
    }
}
