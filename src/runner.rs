use std::io::{self, BufRead};


#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BamRecord {
    pub raw_data: Vec<u8>, //add decoded fields here???
}

/// Represents the BAM file header.
#[derive(Debug, Default, Clone)]
pub struct BamHeader {
    pub raw_data: Vec<u8>, //when do i fill this? decode right away?
}


pub trait Statistic {
    fn process(&mut self, record: &BamRecord);
    fn finalize(&self) -> String; // Optional: make outputable
}


pub struct WorkflowRunner {
    pub stats: Vec<Box<dyn Statistic>>,
}

impl WorkflowRunner {
    /// Creates a new runner.
    pub fn new() -> Self {
        Self { stats: Vec::new() }
    }

    /// Registers a new statistic to the workflow.
    pub fn add_statistic(&mut self, stat: Box<dyn Statistic>) {
        self.stats.push(stat);
    }

    /// Processes BAM records from a buffered reader.
    pub fn process<R: BufRead>(&mut self, mut reader: R) -> io::Result<()> {
        let mut record = BamRecord::default();
        while Self::parse_record(&mut reader, &mut record)? {
            for stat in self.stats.iter_mut() {
                stat.process(&record);
            }
        }
        Ok(())
    }

    /// Returns Ok(true) if a record was read, Ok(false) if EOF.
    fn parse_record<R: BufRead>(reader: &mut R, record: &mut BamRecord) -> io::Result<bool> {
        // TODO: Replace with actual BAM parsing logic
        let mut buffer = Vec::new();
        let bytes_read = reader.read_until(b'\n', &mut buffer)?;
        if bytes_read == 0 {
            return Ok(false); // EOF
        }
        record.raw_data = buffer;
        Ok(true)
    }

    /// Consumes the runner and returns the final statistics.
    pub fn finalize(self) -> Vec<Box<dyn Statistic>> {
        self.stats
    }
}
