mod my_hashmap;

use std::collections::HashMap;
use crate::ranker::my_hashmap::MyMap;

pub fn map_f(name: String, content: String) -> HashMap<String, u64> {
    let mut mp = MyMap::new();
    for pt in content.rsplit(|ch| ch == ' ' || ch == '\n' || ch == '\r') {
        // divide by space and line feed.
        // add '\r' to cope with CRLF files.
        mp.insert(pt.to_string(), 1);
    }
    mp.unpack()
}

pub fn reduce_f(key: String, values: Vec<u64>) -> (String, u64) {
    (key, values.into_iter().sum())
}