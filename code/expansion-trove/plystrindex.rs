// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

use core::common::*;
use std::cmp::Ordering;
use std::collections::BTreeMap;

// TODO write index structures
pub type PlayerInd = specs::world::Index;
pub type StructInd = specs::world::Index;

// Vector of player structure combinations
pub type PlayerStructVec = Vec<Playerstructpair>;
// BTreeMap of structure player combinations
pub type StructPlayerMap = BTreeMap<StructInd, PlayerInd>;

#[derive(Debug)]
pub struct Playerstructpair {
    player: PlayerInd,
    structure: StructInd,
}

//assumption 1: every player can have 0 or more stations
//assumption 2: every station is owned by just one player
#[derive(Debug)]
pub struct Playerstructindex {
    psi: PlayerStructVec,
    spi: StructPlayerMap,
}

impl Playerstructindex {
    // create new Index
    pub fn new() -> Playerstructindex {
        let psi = PlayerStructVec::new();
        let spi = StructPlayerMap::new();
        Playerstructindex { psi: psi, spi: spi }
    }

    // add entry for player
    pub fn add_station(&mut self, player: PlayerInd, structure: StructInd) {
        // if there is no entry for a station, then go on, otherwise ignore yet
        if self.spi.get(&structure) == None {
            self.spi.insert(structure, player);
            //new player struct pair
            let psp = Playerstructpair {
                player: player,
                structure: structure,
            };
            //push into vector
            self.psi.push(psp);
            //and sort the vector, so that I can use binary search
            self.psi.sort();
        }
    }

    //get player for structure, None if the player doesn't have a station
    pub fn get_player(&self, structure: StructInd) -> Option<&PlayerInd> {
        self.spi.get(&structure)
    }

    // remove a structure
    //TODO write test for remove record
    pub fn remove_record(&mut self, structure: StructInd) {
        match self.get_player(structure) {
            //TODO should return None
            None => return,
            Some(&player) => {
                let psp = Playerstructpair {
                    player: player,
                    structure: structure,
                };
                match self.psi.binary_search(&psp) {
                    //TODO should return None
                    Err(_) => return,
                    Ok(index) => {
                        self.psi.remove(index);
                        self.spi.remove(&structure);
                    }
                }
            }
        }
    }

    // COMBAK all the structures of a player
    fn get_structures(&self, _player: PlayerInd) {
        println!("test",);
    }
}

//impl traits for Playerstructpair
impl PartialEq for Playerstructpair {
    fn eq(&self, other: &Playerstructpair) -> bool {
        if (self.player == other.player) && (self.structure == other.structure) {
            return true;
        }
        false
    }
}

impl Eq for Playerstructpair {}

impl PartialOrd for Playerstructpair {
    fn partial_cmp(&self, other: &Playerstructpair) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// first sort for player id and then the structure id
impl Ord for Playerstructpair {
    fn cmp(&self, other: &Playerstructpair) -> Ordering {
        if self.eq(other) {
            return Ordering::Equal;
        } else if self.player == other.player {
            if self.structure > other.structure {
                return Ordering::Greater;
            } else {
                return Ordering::Less;
            }
        } else if self.player > other.player {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_testdata() -> Playerstructindex {
        let mut psi = Playerstructindex::new();
        //record 3
        psi.add_station(100, 1001);
        // record shouldn't be added
        psi.add_station(2, 1000);
        //record 0
        psi.add_station(1, 1000);
        //record 1
        psi.add_station(1, 1002);
        //record 2
        psi.add_station(2, 2003);
        psi
    }

    #[test]
    fn get_player_for_station() {
        let psi = create_testdata();
        let player = psi.get_player(2003).unwrap();
        assert_eq!(2, *player);
    }

    #[test]
    fn get_player_for_non_existing_station() {
        let psi = create_testdata();
        let player = psi.get_player(25);
        assert_eq!(None, player);
    }

    #[test]
    fn add_structure_deny_when_structure_already_exists() {
        let psi = create_testdata();
        let record = Playerstructpair {
            player: 2,
            structure: 2003,
        };
        let element = psi.psi.binary_search(&record);
        assert_eq!(element, Ok(2));
    }
    #[test]
    fn playerstructindex_sorttest() {
        //init
        let mut psi = Playerstructindex::new();
        psi.add_station(100, 1001);
        psi.add_station(2, 1002);
        //sorting ok?
        let result = psi.psi[0] < psi.psi[1];
        assert_eq!(result, true);
        // one more element
        psi.add_station(2, 2003);

        //is record 0 what we expect?
        let result = &psi.psi[0];
        let record = Playerstructpair {
            player: 2,
            structure: 1002,
        };
        assert_eq!(*result, record);
        //is record 1 what we expect?
        let result = &psi.psi[1];
        let record = Playerstructpair {
            player: 2,
            structure: 2003,
        };
        assert_eq!(*result, record);
        //is record 2 what we expect?
        let result = &psi.psi[2];
        let record = Playerstructpair {
            player: 100,
            structure: 1001,
        };
        assert_eq!(*result, record);
    }

    #[test]
    fn binary_search_playerstructure() {
        let psi = create_testdata();
        let record = Playerstructpair {
            player: 2,
            structure: 2003,
        };
        let element = psi.psi.binary_search(&record);
        assert_eq!(element, Ok(2));
    }

    #[test]
    fn playerstructpair_eq() {
        let left = Playerstructpair {
            player: 1,
            structure: 1000,
        };
        let right = Playerstructpair {
            player: 1,
            structure: 1000,
        };
        assert_eq!(left, right);
        let result = left == right;
        assert_eq!(result, true);
    }
    #[test]
    fn playerstructpair_neq_player() {
        let left = Playerstructpair {
            player: 1,
            structure: 1000,
        };
        let right = Playerstructpair {
            player: 2,
            structure: 1000,
        };
        assert_ne!(left, right);
    }
    #[test]
    fn playerstructpair_neq_structure() {
        let left = Playerstructpair {
            player: 1,
            structure: 1000,
        };
        let right = Playerstructpair {
            player: 1,
            structure: 2001,
        };
        assert_ne!(left, right);
    }
    #[test]
    fn playerstructpair_order_player() {
        let left = Playerstructpair {
            player: 3,
            structure: 1000,
        };
        let right = Playerstructpair {
            player: 2,
            structure: 2000,
        };
        let result = left < right;
        assert_eq!(result, false);
        let result = left > right;
        assert_eq!(result, true);
    }
    #[test]
    fn playerstructpair_order_structure() {
        let left = Playerstructpair {
            player: 1,
            structure: 1000,
        };
        let right = Playerstructpair {
            player: 1,
            structure: 2000,
        };
        let result = left < right;
        assert_eq!(result, true);
        let result = left > right;
        assert_eq!(result, false);
    }
}
