use noodles_bam::{io::Reader, record::Record};
use noodles_sam as sam;
use std::fs::File;
use std::io::{self, BufReader};

#[derive(Debug, Default, Clone)]
pub struct BamHeader {
    pub header: sam::Header,
}

pub trait Statistic {
    fn process(&mut self, record: &Record, header: &sam::Header) -> io::Result<bool>;
    fn finalize(&self) -> String;
}

pub struct WorkflowRunner {
    pub stats: Vec<Box<dyn Statistic>>,
    pub header: Option<BamHeader>,
}

impl WorkflowRunner {
    pub fn new() -> Self {
        Self {
            stats: Vec::new(),
            header: None,
        }
    }

    pub fn add_statistic(&mut self, stat: Box<dyn Statistic>) {
        self.stats.push(stat);
    }

    pub fn decode_header<R: io::Read>(&mut self, reader: &mut Reader<R>) -> io::Result<()> {
        let header = reader.read_header()?;
        self.header = Some(BamHeader { header });
        Ok(())
    }

    pub fn process(&mut self, file_path: &str) -> io::Result<()> {
        let file = File::open(file_path)?;
        let mut reader = Reader::new(BufReader::new(file));
        self.decode_header(&mut reader)?;

        let mut record = Record::default();

        while reader.read_record(&mut record)? != 0 {
            let mut accepted = true;
        
            for stat in self.stats.iter_mut() {
                if let Some(header) = &self.header {
                    if !stat.process(&record, &header.header)? {
                        accepted = false;
                        break;
                    }
                }
            }
        
            if !accepted {
                continue;
            }
        }

        Ok(())
    }

    pub fn finalize(self) -> Vec<Box<dyn Statistic>> {
        self.stats
    }
}