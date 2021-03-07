use anyhow::Result;
use log::*;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::ops::AddAssign;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

type MapFn = fn(String, String) -> HashMap<String, u64>;

pub fn map(rx: Receiver<String>, map_f: MapFn, worker_num: usize) -> Result<usize> {
    let pool = ThreadPool::new(worker_num);
    let counter = Arc::new(Mutex::new(0_usize));
    for s in rx {
        let counter = counter.clone();
        pool.execute(move || {
            let mut num = 0;
            {
                let mut c = counter.lock().unwrap();
                num = *c;
                *c += 1;
            }
            let f = File::create(format!("/tmp/URanker/map-{}", num)).unwrap();
            let mut w = BufWriter::new(f);
            let hm = map_f(String::new(), s);
            for kv in hm {
                w.write(serde_json::to_string(&kv).unwrap().as_bytes());
                w.write(b"\n");
            }
            info!("{}th mapper finished", num);
        });
    }

    pool.join();
    let c = counter.lock().unwrap();

    Ok(*c)
}

type ReduceFn = fn(usize, usize, usize) -> Option<HashMap<String, u64>>;

pub fn reduce(counter: usize, reduce_f: ReduceFn, worker_num: usize) -> Result<usize> {
    let pool = ThreadPool::new(worker_num);
    for i in 0..worker_num {
        let counter = counter;
        pool.execute(move || {
            let o_file = File::create(format!("/tmp/URanker/reduce-{}", i)).unwrap();
            let mut writer = BufWriter::new(o_file);
            let mut reduced = HashMap::new();

            match reduce_f(i, worker_num, counter) {
                Some(hm) => reduced = hm,
                _ => {
                    panic!("serde sucks");
                }
            }

            let mut v = reduced.into_iter().collect::<Vec<(String, u64)>>();
            v.sort_by(|a, b| b.1.cmp(&a.1));

            for kv in v {
                write!(writer, "{}\n", serde_json::to_string(&kv).unwrap());
            }
        })
    }

    pool.join();
    Ok(worker_num)
}
