mod d_writer;

use std::io::{BufReader, BufWriter, prelude::*, Cursor};
use std::fs::{File};
use serde_json;
use anyhow::Result;
use log;
use d_writer::DWriter;

#[derive(Debug)]
pub struct MyReader {
    source: File,
    buf_vec: Vec<File>,
    buf_size: u16,
    buf_writer: DWriter<File>,
    finished: bool
}

impl MyReader {
    fn split_file(&mut self, file: &str, div: u8) -> Result<()> {
        let i_file = File::open(file)?;
        let mut cursor = Cursor::new(i_file);
        
        for read_outcome in cursor.split(div) {
            match read_outcome {
                Some(url) => {
                    self.write(url);
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn write(&mut self, url: Vec<u8>) -> Result<usize> {
        self.buf_writer.write(url.as_slice())
    }

    pub fn read(&mut self, file: File) {

    }
}