mod my_hashmap;

use crate::ranker::my_hashmap::MyMap;
use serde_json::{Deserializer, StreamDeserializer, Value};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hasher;
use std::io::{BufRead, BufReader, Result};
use std::ops::AddAssign;

pub fn map_f(name: String, content: String) -> HashMap<String, u64> {
    let mut mp = MyMap::new();
    for pt in content.rsplit(|ch| ch == ' ' || ch == '\n' || ch == '\r') {
        // divide by space and line feed.
        // add '\r' to cope with CRLF files.
        mp.insert(pt.to_string(), 1);
    }
    mp.unpack()
}

pub fn reduce_f(num: usize, n_worker: usize, counter: usize) -> Option<HashMap<String, u64>> {
    let mut ans = HashMap::new();
    for file in 0..counter {
        ans = append_hs(ans, num, n_worker, file).unwrap();
    }
    if ans.is_empty() {
        None
    } else {
        Some(ans)
    }
}

fn append_hs(
    hs: HashMap<String, u64>,
    worker: usize,
    n_worker: usize,
    nth_file: usize,
) -> Result<HashMap<String, u64>> {
    let path = format!("/tmp/URanker/map-{}", nth_file);
    let mut ans = hs;

    let i_file = File::open(path).unwrap();
    let mut i_file = BufReader::new(i_file);
    for line in i_file.lines() {
        let line = line.unwrap();
        let (k, v) = serde_json::from_str::<(String, u64)>(line.as_str()).unwrap();

        if hash(k.clone()) % n_worker == worker {
            // dbg!(ans.clone());
            ans.entry(k).or_default().add_assign(v);
            // dbg!(ans.clone());
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
