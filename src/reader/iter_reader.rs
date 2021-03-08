//! read to buf

use crate::reader::LONG_LOG;
use log::*;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hasher;
use std::io::{Read, Seek, SeekFrom};
use std::iter::Iterator;

const BUF_SIZE: usize = 6 * 1024 * 1024;
// static BUFFER: [u8; BUF_SIZE] = [b'\0'; BUF_SIZE];

#[derive(Debug)]
/// # Iterate through big file!
/// this magic structure offers a method to iterate through the annoying *GIANT_FILE*.
///
/// as for those long urls, we will record their hash value and address to hashmap and write to file
pub struct IterReader {
    /// source file
    file: File,

    /// next URL's absolute address in file
    read_cur: usize,

    /// url divides by this byte
    divider: u8,

    // maintaining a buf string...
    /// buffer to store bytes
    buf: Box<[u8]>,
    /// length of invalid data in buffer
    len: usize,
    /// where next URL begins in buffer
    cur: usize,
    /// long urls' hash value and it's first appearance place in raw file.
    long_urls: HashMap<u64, (usize, usize)>,
}

impl IterReader {
    /// # make a new instance
    pub fn new<P: Into<String>>(file: P, div: u8) -> std::io::Result<Self> {
        let f = File::open(file.into())?;
        let v = vec![b'\0'; BUF_SIZE];
        Ok(Self {
            file: f,
            read_cur: 0,
            divider: div,
            long_urls: HashMap::new(),
            buf: v.into_boxed_slice(),
            len: 0,
            cur: 0,
        })
    }

    /// # load buffer content from file
    /// replace buffer with following data in file
    fn buf_load(&mut self) -> std::io::Result<usize> {
        self.cur = 0;
        self.len = self.file.read(&mut self.buf.as_mut())?;
        Ok(self.len)
    }

    /// # pick a url from buffer
    /// if can get a full url, return (full URL string, true)
    ///
    /// if cannot get a full url, return (part URL String, false)
    fn buf_pick(
        &mut self,
        s: String,
        // is the previous reading successful?
        is_last_success: bool,
    ) -> (String, bool) {
        let mut prev = b'\0';
        let mut s = String::from(s);
        let file_end = self.len < self.buf.len(); // check if the file reading is ended.

        for i in self.cur..self.len {
            let b = self.buf[i];
            if b == self.divider || (b == b'\0' && file_end) {
                if prev == self.divider || (is_last_success == true && s.is_empty()) {
                    // avoid begin reading *new* URL at dividers
                    // if is_last_success is true means function is reading new URL...
                    self.cur += 1;
                    continue;
                }
                self.cur = i + 1;

                return (s, true);
            }

            s.push(char::from(b));
            prev = b;
        }
        if self.len == 0 {
            // because always buffer_loaded before using buffer_pick
            // add such patch works I guess...
            return (s, true);
        }
        (s, false)
    }
}

impl Iterator for IterReader {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let s = String::new();
        if self.cur + 1 > self.len {
            if self.buf_load().unwrap() == 0 {
                // eventually end reading...
                return None;
            }
        }
        // check whether have divider in buffer
        let (s, is_full) = self.buf_pick(s, true);
        if is_full {
            self.read_cur += s.len() + 1;
            return Some(s);
        }
        // have no divider in buffer
        self.buf_load().unwrap();

        let (s, is_full) = self.buf_pick(s, is_full);
        if is_full {
            self.read_cur += s.len() + 1;
            return Some(s);
        }

        // the url occupies more than 2 buffers, which means it's a long url.
        // load and correct long url recorder
        let mut recorder = File::open(LONG_LOG).unwrap();
        recorder.seek(SeekFrom::End(0)).unwrap();

        let mut hs = DefaultHasher::new();
        hs.write(s.clone().as_bytes());
        let mut s_len = s.len();

        loop {
            self.buf_load().unwrap();
            let s = String::new();
            let (s, is_full) = self.buf_pick(s, is_full);
            s_len += s.len();
            hs.write(s.as_bytes());
            if is_full {
                break;
            }
            self.buf_load().unwrap();
        }
        let hs_val = hs.finish();
        let s = format!("uranker://{}", hs_val);
        self.long_urls
            .entry(hs_val)
            .or_insert((self.read_cur, s_len));
        self.read_cur += s_len + 1;

        Some(s)
    }
}

impl Drop for IterReader {
    fn drop(&mut self) {
        if !self.long_urls.is_empty() {
            let mut f = File::create(LONG_LOG).unwrap();
            serde_json::to_writer(f, &self.long_urls).unwrap();
        }
        info!("IterReader Dropped!");
    }
}

#[cfg(test)]
mod tests {
    use crate::reader::iter_reader::IterReader;

    #[test]
    fn test_iter() {
        // you can create test file by executing bash script like below:
        /*
         for i in {0..<Value >= BUF_SIZE>} # Value should big enough to test large file reading ability...
         do
            echo "rc-$i" > foo.txt
         done
        */

        let ir = IterReader::new("foo.txt", b'\n').unwrap();
        let mut i = 1;
        for record in ir {
            assert_eq!(record, format!("rc-{}", i));
            i += 1;
        }
    }
}
