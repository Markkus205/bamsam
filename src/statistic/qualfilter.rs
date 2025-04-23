use std::io;
use noodles_bam::record::Record;
use noodles_sam::alignment::record::QualityScores;
use crate::runner::Statistic;


pub struct QualityFilterStat {
    pub min_quality: f32,
    pub filtered_out: usize,
}

impl QualityFilterStat {
    pub fn new(min_quality: f32) -> Self {
        Self {
            min_quality,
            filtered_out: 0,
        }
    }
}

impl Statistic for QualityFilterStat {
    fn process(&mut self, record: &Record, _header: &noodles_sam::Header) -> io::Result<bool> {
        let qualities = record.quality_scores();
        if qualities.is_empty() {
            self.filtered_out += 1;
            return Ok(false);
        }

        let sum: u32 = qualities
            .iter()
            .map(|q| q.map(|v| v as u32))
            .sum::<Result<u32, io::Error>>()?;

        let avg = sum as f32 / qualities.len() as f32;

        if avg < self.min_quality {
            self.filtered_out += 1;
            Ok(false)
        } else {
            Ok(true)
        }
    }

    fn finalize(&self) -> String {
        format!("Filtered out {} reads with avg quality < {}", self.filtered_out, self.min_quality)
    }
}
