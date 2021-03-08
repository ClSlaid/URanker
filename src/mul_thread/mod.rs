mod mul_reader;

use crate::mul_thread::mul_reader::MulReader;
use crate::reader::LONG_LOG;
use anyhow::Result;
use log::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{copy, BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Split, Write};
use std::str::FromStr;
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

type ReduceFn = fn(usize, usize, usize) -> Option<Vec<(String, u64)>>;

pub fn reduce(counter: usize, reduce_f: ReduceFn, worker_num: usize) -> Result<usize> {
    let pool = ThreadPool::new(worker_num);
    for i in 0..worker_num {
        let counter = counter;
        pool.execute(move || {
            let o_file = File::create(format!("/tmp/URanker/reduce-{}", i)).unwrap();
            let mut writer = BufWriter::new(o_file);
            let mut v = Vec::new();

            match reduce_f(i, worker_num, counter) {
                Some(hm) => v = hm,
                _ => {
                    panic!("serde sucks");
                }
            }

            for kv in v {
                write!(writer, "{}\n", serde_json::to_string(&kv).unwrap());
            }
        })
    }

    pool.join();
    Ok(worker_num)
}

pub fn rank<P: Into<String>>(source: P, reduce_num: usize) {
    let reader = MulReader::new(reduce_num);
    let long = File::open(LONG_LOG);
    let s = source.into();
    let mut writer = BufWriter::new(File::create("report.csv").unwrap());
    let mut long_map = HashMap::<u64, (u64, u64)>::new();
    match long {
        Ok(long_log) => {
            long_map = serde_json::from_reader(long_log).unwrap();
            true
        }
        _ => false,
    };

    writer.write("URL, Frequency\n".as_bytes());
    let mut counter = 0;
    for (url, val) in reader {
        if counter >= 100 {
            break
        }
        if !long_map.is_empty() && url.starts_with("uranker://") {
            write_long(s.clone(), long_map.clone(), url, &mut writer);
        } else {
            writer.write(format!("{}, {}\n", url, val).as_bytes());
        }
        counter += 1;
    }
}

fn write_long(
    source: String,
    long_map: HashMap<u64, (u64, u64)>,
    hashed: String,
    writer: &mut BufWriter<File>,
) {
    // drop "uranker://"
    let hash_val = hashed[10..].parse::<u64>().unwrap();

    // long URL's position and its length in source file.
    let (url_pos, url_len) = long_map.get(&hash_val).unwrap();

    let mut source = File::open(source).unwrap();

    let mut buffer = ['\0'; 50];
    source.seek(SeekFrom::Start(*url_pos));
    let mut handle = source.take(*url_len);

    copy(&mut handle, writer);
}
