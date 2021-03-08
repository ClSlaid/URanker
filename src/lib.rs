#![feature(shrink_to)]

pub mod mul_thread;
pub mod ranker;
pub mod reader;

pub use reader::SourceReader;

pub use ranker::map_f;
pub use ranker::reduce_f;
