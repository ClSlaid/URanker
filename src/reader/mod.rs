// read file and pass it to main thread in sequence
mod iter_reader;
mod tests;

use crate::reader::iter_reader::IterReader;
use anyhow::Result;
use log::*;
use std::io::{BufRead, BufReader};
use std::sync::mpsc;

const BUF_MAX: usize = 15 * 1024 * 1024; // 15 MB

#[derive(Debug)]
pub struct MyReader {
    source: Option<IterReader>,
    div: u8,
    buf_pipe: mpsc::SyncSender<String>,
    buffer: String,
}

impl MyReader {
    // spawn a new reader instance
    pub fn new(file: &str, div: u8, buf_pipe: mpsc::SyncSender<String>) -> Result<Self> {
        Ok(MyReader {
            source: Some(IterReader::new(file, div)?),
            div,
            buf_pipe,
            buffer: String::new(),
        })
    }

    // split file into little files, no more than 10 files.
    pub fn split_file(mut self) -> Result<()> {
        // for read_outcome in self.source.into_inner() {
        // self.write(read_outcome);
        // }
        let splits = self.source.take().unwrap();
        for s in splits {
            self.write(s).unwrap();
        }
        Ok(())
    }

    // write to buffer.
    // return how many bytes written into file.
    fn write<S: Into<String>>(&mut self, url: S) -> Result<usize> {
        // if a write buffer is full, send it to the main thread.
        let url = url.into();
        if url.len() > BUF_MAX - self.buffer.len() - 1 {
            self.buf_pipe.send(self.buffer.clone())?;
            self.buffer = String::new();
        }

        // Buffer is can hold the content, add it to buffer.
        self.buffer += url.as_str();
        self.buffer.push(char::from(self.div));

        return Ok(url.len());
    }
}

impl Drop for MyReader {
    fn drop(&mut self) {
        self.buf_pipe.send(self.buffer.clone());
        self.buffer.clear();
        info!("MyReader dropped!");
    }
}
