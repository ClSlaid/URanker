mod my_hashmap;
mod my_vec;

use crate::ranker::my_hashmap::MyMap;
use serde_json::{Deserializer, StreamDeserializer, Value};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hasher;
use std::io::{BufRead, BufReader, Result};
use std::ops::AddAssign;
use crate::ranker::my_vec::MyVec;

pub fn map_f(name: String, content: String) -> HashMap<String, u64> {
    let mut mp = MyMap::new();
    for pt in content.rsplit(|ch| ch == ' ' || ch == '\n' || ch == '\r') {
        // divide by space and line feed.
        // add '\r' to cope with CRLF files.
        mp.insert(pt.to_string(), 1);
    }
    mp.unpack()
}

pub fn reduce_f(num: usize, n_worker: usize, counter: usize) -> Option<Vec<(String, u64)>> {
    let mut ans = MyVec::new();
    for file in 0..counter {
        ans = append_from_file(ans, num, n_worker, file).unwrap();
    }
    let v = ans.unpack();
    if v.is_empty() {
        None
    } else {
        Some(v)
    }
}

fn append_from_file(
    hs: MyVec,
    worker: usize,
    n_worker: usize,
    nth_file: usize,
) -> Result<MyVec> {
    let path = format!("/tmp/URanker/map-{}", nth_file);
    let mut ans = hs;

    let i_file = File::open(path).unwrap();
    let mut line_num = 0_u64;
    let mut i_file = BufReader::new(i_file);
    for line in i_file.lines() {
        let line = line.unwrap();
        let elem = serde_json::from_str::<(String, u64)>(line.as_str()).unwrap();
        line_num += 1;

        if hash(elem.clone().0) % n_worker == worker {
            ans.insert(elem);
        }

    }
    Ok(ans)
}



fn hash(s: String) -> usize {
    let mut hasher = DefaultHasher::new();
    let s = s.clone();
    hasher.write(s.as_bytes());
    hasher.finish() as usize
}
