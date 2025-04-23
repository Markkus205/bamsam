use std::io;
use noodles_bam::record::Record;
use crate::runner::Statistic;
use noodles_sam::Header;  

pub struct LineCount {
    pub count: usize,
}

impl LineCount {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

impl Statistic for LineCount {
    fn process(&mut self, _record: &Record, _header: &Header) -> io::Result<bool> {
        self.count += 1;
        Ok(true) 
    }

    fn finalize(&self) -> String {
        format!("Total reads: {}", self.count)
    }
}
