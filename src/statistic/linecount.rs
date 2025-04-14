use crate::runner::Statistic;
use crate::runner::BamRecord;

pub struct LineCount {
    pub count: usize,
}

impl LineCount {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

impl Statistic for LineCount {
    fn process(&mut self, _record: &BamRecord) {
        self.count += 1;
    }

    fn finalize(&self) -> String {
        format!("Total reads: {}", self.count)
    }
}

//in theory tests
