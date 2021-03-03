//! # Dynamic Writer
//! Dynamic Writer will automatically check if writing to
use std::io::{BufWriter, Result, Write};
use std::fs::File;

static MAX_SIZE: u32 = u32::MAX;

#[derive(Debug)]
pub struct DWriter<T> {
    counter: u32,
    writer: BufWriter<T>,
    written: u32
}

impl DWriter<T> {
    pub fn new() -> Result<DWriter<T>> {
        let f = File::create("/tmp/URanker-split-0")?;
        let writer = BufWriter::new(f);

        Ok(Self {counter: 0, writer, written: 0})
    }
    pub fn write(&mut self, bytes: &[u8]) -> Result<usize> {
        if bytes.len() as u64 + self.written as u64 > MAX_SIZE as u64{
            self.writer.flush();
            counter += 1;
            let f = File::create(format!("/tmp/URanker-split-{}", counter))?;
            self.writer = BufWriter::new(f);
            self.written = 0;
        }
        self.writer.write(bytes);
        self.written += bytes.len();
        Ok(bytes.len())
    }
}