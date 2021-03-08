use serde_json::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct MulReader {
    files: Vec<BufReader<File>>,
    buf: Vec<Option<(String, u64)>>,
}

impl MulReader {
    pub fn new(reduce_num: usize) -> Self {
        let mut reader_vec = Vec::new();
        let mut buf_vec = Vec::new();
        for i in 0..reduce_num {
            let i_file = File::open(format!("/tmp/URanker/reduce-{}", i)).unwrap();
            let mut reader = BufReader::new(i_file);

            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            let kv = serde_json::from_str(line.as_str()).unwrap();

            buf_vec.push(Some(kv));
            reader_vec.push(reader);
        }

        Self {
            files: reader_vec,
            buf: buf_vec,
        }
    }

    fn pick(&mut self, i: usize) -> Option<(String, u64)> {
        let mut s = String::new();
        self.files[i].read_line(&mut s).unwrap();
        let kv = match serde_json::from_str::<(String, u64)>(s.as_str()) {
            Ok(kv) => Some(kv),
            Err(_) => None,
        };
        let ans = self.buf[i].clone();
        self.buf[i] = kv;
        return ans;
    }
}

impl Iterator for MulReader {
    type Item = (String, u64);
    fn next(&mut self) -> Option<Self::Item> {
        let mut max_index = 0_usize;
        let mut max_val = 0_u64;
        for i in 0..self.buf.len() {
            match self.buf.get(i) {
                Some(Some((k, v))) => {
                    if *v > max_val {
                        max_val = *v;
                        max_index = i;
                    }
                }
                _ => continue,
            }
        }

        self.pick(max_index)
    }
}
