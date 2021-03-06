// read file and pass it to main thread in sequence
mod iter_reader;

use std::sync::mpsc;
use anyhow::{Result};
use std::io::{BufReader, BufRead};
use crate::reader::iter_reader::IterReader;

const BUF_MAX: usize = 50 * 1024 * 1024; // 50 MB

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
        for s in splits{
            self.write(s).unwrap();
        }
        Ok(())
    }

    // write to buffer.
    // return how many bytes written into file.
    fn write <S: Into<String>> (&mut self, url: S) -> Result<usize> {

        // if a write buffer is full, send it to the main thread.
        let url = url.into();
        if url.len() > BUF_MAX - self.buffer.len() - 1 {
            self.buf_pipe.send(self.buffer.clone())?;
            self.buffer = String::new();
        }

        // Buffer is can hold the content, add it to buffer.
        self.buffer += url.as_str();
        self.buffer.push(char::from(self.div));

        return Ok(url.len())
    }
}
