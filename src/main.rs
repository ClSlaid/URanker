//! # URanker's main functions...
//! As you see, it's mainly adapted from 6.824's mrsequencial.go

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::hash::Hasher;
use std::io::{BufReader, BufWriter};
use std::ops::AddAssign;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use clap::{App, Arg, SubCommand};
use log::*;
use threadpool::ThreadPool;
use uranker::mul_thread::{map, reduce, rank};
use uranker::{map_f, reduce_f, SourceReader};
use std::fs::File;

// number of reducers
const MAPPER_NUM: usize = 4;
const REDUCER_NUM: usize = 4;

// working directory
const DIR: &'static str = "/tmp/URanker";

fn my_hash<T: Into<Vec<u8>>>(s: T) -> usize {
    let mut hs = DefaultHasher::new();
    hs.write(s.into().as_slice());
    hs.finish() as usize
}

fn startup(file: &str) {
    // clean up temporary files
    fs::create_dir(DIR);
    fs::remove_dir_all(DIR);
    fs::create_dir(DIR);

    // a 10 file sized queue
    let (tx, rx) = mpsc::sync_channel(10);
    let mut reader = SourceReader::new(file, b'\n', tx).unwrap();
    let rt = thread::spawn(move || reader.split_file());

    // map phase
    let counter = map(rx, map_f, MAPPER_NUM).unwrap();
    rt.join();

    // reduce phase
    let reduced = reduce(counter, reduce_f, REDUCER_NUM).unwrap();

    // rank phase
    rank(file, reduced);
}
fn main() {
    let matches = App::new("URanker")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .value_name("FILE")
                .help("Path to target file")
                .takes_value(true),
        )
        .get_matches();

    let file = matches.value_of("path");
    startup(file.unwrap());
}
