//! # HashMap for duplicated keys
//! `MyMap` will check duplicated keys on every insertion.
//!
//! If the key is duplicated, values will be added together.

use std::collections::HashMap;
use std::ops::AddAssign;

type HM = HashMap<String, u64>;

pub struct MyMap {
    map: HM,
}

impl MyMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn from(map: HM) -> Self {
        Self { map }
    }
    pub fn join(map1: MyMap, map2: MyMap) -> Self {
        let mut new_map = MyMap::new();
        for (key, value) in map1.map.into_iter().chain(map2.map.into_iter()) {
            new_map.insert(key, value);
        }

        new_map
    }

    pub fn unpack(self) -> HM {
        self.map
    }
    pub fn insert(&mut self, key: String, value: u64) {
        self.map.entry(key).or_default().add_assign(value);
    }
}
