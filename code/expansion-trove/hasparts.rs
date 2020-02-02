// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information

use core::common::stdenums::*;
use specs;
use std::cmp::Ordering;

// structure describing each part
#[derive(Debug)]
pub struct Part {
    objtype: ObjType,
    index: specs::world::Index,
}

pub type PartsType = Vec<Part>;

// Parts of a bigger structure
// use in stations and modules
#[derive(Debug)]
pub struct HasParts {
    parts: PartsType,
}

impl HasParts {
    pub fn new() -> Self {
        let parts = PartsType::new();
        HasParts { parts }
    }

    pub fn add(&mut self, objtype: ObjType, index: specs::world::Index) {
        if self.find(index) == None {
            self.parts.push(Part { objtype, index });
            self.parts.sort();
        }
    }

    pub fn find(&self, id: specs::world::Index) -> Option<usize> {
        let result = self.parts.iter().position(|&ref r| r.index == id);
        result
    }

    pub fn remove(&mut self, id: specs::world::Index) -> Option<usize> {
        let toremove = self.find(id);
        if toremove != None {
            self.parts.remove(toremove.unwrap());
            self.parts.sort();
        }
        toremove
    }
}

impl specs::Component for HasParts {
    type Storage = specs::VecStorage<HasParts>;
}

//impl traits for Playerstructpair
impl PartialEq for Part {
    fn eq(&self, other: &Part) -> bool {
        if self.index == other.index {
            true
        } else {
            false
        }
    }
}

impl Eq for Part {}

impl PartialOrd for Part {
    fn partial_cmp(&self, other: &Part) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// first sort for Parts
impl Ord for Part {
    fn cmp(&self, other: &Part) -> Ordering {
        if self.eq(other) {
            return Ordering::Equal;
        } else {
            if self.index > other.index {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use specs::prelude::*;
    use specs::world::Builder;

    #[test]
    fn create_partof_component() {
        let mut world = specs::World::new();
        world.register::<HasParts>();
        world.create_entity().with(HasParts::new()).build();
    }

    fn create_testdata() -> HasParts {
        let mut part = HasParts::new();
        //record 0
        part.add(ObjType::Owner, 1);
        // record shouldn't be added
        part.add(ObjType::Owner, 1);
        //record 2
        part.add(ObjType::Module, 1002);
        //record 3
        part.add(ObjType::Module, 2003);
        //record 1
        part.add(ObjType::Station, 1000);
        part
    }

    #[test]
    fn get_sorttest() {
        let testdata = create_testdata();
        assert_eq!(testdata.find(0001), Some(0));
        assert_eq!(testdata.find(1000), Some(1));
        assert_eq!(testdata.find(1002), Some(2));
        assert_eq!(testdata.find(2003), Some(3));
    }

    #[test]
    fn binary_search_parts() {
        let testdata = create_testdata();
        let record = Part {
            objtype: ObjType::Owner,
            index: 1,
        };
        let element = testdata.parts.binary_search(&record);
        assert_eq!(element, Ok(0));
    }

    #[test]
    fn get_vecindex_for_non_existing_index() {
        let testdata = create_testdata();
        let offrecord = testdata.find(25);
        assert_eq!(None, offrecord);
    }

    #[test]
    fn remove_from_vec() {
        let mut testdata = create_testdata();
        testdata.remove(5000);
        assert_eq!(testdata.find(2003), Some(3));
        testdata.remove(1002);
        assert_eq!(testdata.find(2003), Some(2));
        testdata.remove(2003);
        assert_eq!(testdata.find(1000), Some(1));
    }
}
