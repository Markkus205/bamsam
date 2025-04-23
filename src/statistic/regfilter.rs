use noodles_bam::record::Record;
use noodles_sam as sam;
use crate::runner::Statistic;
use std::io;

pub struct RegionFilterStat {
    chromosome: String,
    start: usize,
    end: usize,
    filtered_out: usize,
}

impl RegionFilterStat {
    pub fn new(chromosome: String, start: usize, end: usize) -> Self {
        Self {
            chromosome,
            start,
            end,
            filtered_out: 0,
        }
    }
}

impl Statistic for RegionFilterStat {
    fn process(&mut self, record: &Record, header: &sam::Header) -> io::Result<bool> {
        // Get the reference sequence name. Check documentation again there has to be a better way.
        //Maybe preprocess part of this and save in header struct? But what if i dont need it..?
        let ref_name = match record.reference_sequence_id().transpose()? {
            Some(id) => header
                .reference_sequences()
                .get_index(id)
                .map(|(name, _)| String::from_utf8_lossy(name).to_string()),
            None => None,
        };
    
        // Get alignment start position as usize
        let pos = record.alignment_start().transpose()?.map(|p| p.get());

    
        // We need to calculate end position. using equal length? prob easiest. Or maybe cigar in case only a none match is in the region?
        if ref_name.as_deref() == Some(&self.chromosome)
            && pos.map_or(false, |p| p >= self.start && p <= self.end)
        {
            return Ok(true); 
        }
    

        self.filtered_out += 1;
        Ok(false)
    }
    

    fn finalize(&self) -> String {
        format!("Filtered out {} reads outside the region {}", self.filtered_out, self.chromosome)
    }
}

   