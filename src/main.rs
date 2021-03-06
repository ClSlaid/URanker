#![feature(map_into_keys_values)]
//! # URanker's main functions...
//! As you see, it's mainly adapted from 6.824's mrsequencial.go

use std::env;
use std::thread;
use std::sync::{mpsc, Mutex, Arc};
use std::fs;
use std::hash::{Hasher};
use std::io::{BufWriter, BufReader};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::ops::AddAssign;

use clap::{App, SubCommand};
use uranker::MyReader;
use uranker::{map_f};
use threadpool::ThreadPool;

// number of reducers
const REDUCER_NUM: usize = 8;

// working directory
const DIR: &'static str = "/tmp/URanker";

fn my_hash<T: Into<Vec<u8>>>(s: T) -> usize {
    let mut hs = DefaultHasher::new();
    hs.write(s.into().as_slice());
    hs.finish() as usize
}

fn startup(file: &str) {

    // clean up temporary files
    fs::remove_dir_all(DIR);
    fs::create_dir(DIR);

    // a 10 file sized queue
    let (tx, rx) = mpsc::sync_channel(10);
    let mut reader = MyReader::new(file, b'\n', tx).unwrap();
    let rt = thread::spawn(move ||{
        reader.split_file()
    });
    let pool = ThreadPool::new(4);

    // map phase

    // give threads unique numbers with mutex. ugly...
    let counter = Arc::new(Mutex::new(0_u32));

    for buffer in rx {
        let counter = Arc::clone(&counter);
        // map buffers and write outcomes to file
        pool.execute(move || {

            let mut map_num = 0;
            {
                let mut num = counter.lock().unwrap();
                map_num = *num;
                *num += 1;
            }

            let mut o_file = BufWriter::new(
                fs::File::create(
                    format!("{}/map-{}",DIR, map_num)
                ).unwrap()
            );

            let kvs = map_f(String::new(), buffer);

            serde_json::to_writer(o_file, &kvs).unwrap();
        });
    }

    pool.join();
    rt.join();

    // reduce phase
    for i in 0..REDUCER_NUM {
        let counter = *counter.lock().unwrap();
        pool.execute(move || {
            let reduce_num = i;
            let o_f = fs::File::create(format!("{}/reduce-{}", DIR, reduce_num)).unwrap();
            let mut writer = BufWriter::new(o_f);
            let mut reduce_kvs: HashMap<String, u64> = HashMap::new();
            for map_no in 0..counter {
                let i_f = fs::File::open(format!("{}/reduce-{}", DIR, map_no)).unwrap();
                let reader = BufReader::new(i_f);
                let json: HashMap<String, u64> = serde_json::from_reader(reader).unwrap();

                for (k, v) in json {
                    if my_hash(k.clone()) % REDUCER_NUM != i {
                        continue;
                    }
                    reduce_kvs.entry(k)
                        .or_insert(v)
                        .add_assign(v);
                }
            }
            let mut reduce_vec: Vec<(&String, &u64)> = reduce_kvs.iter().collect();
            reduce_vec.sort_by(|kva, kvb| kvb.1.cmp(kva.1));
            serde_json::to_writer(writer, &reduce_vec);
        });
    }

    pool.join();


    // rank phase


}
fn main(){
    let matches = App::new("URanker")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(SubCommand::with_name("file")
            .about("URL datasets")
        )
        .get_matches();

    let file = matches.value_of("file");
    startup(file.unwrap());
}