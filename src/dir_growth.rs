use std::collections::BTreeMap;
use chrono::offset::Utc;
use chrono::{Datelike, DateTime};
use human_bytes::human_bytes;
use termsize;
use walkdir::WalkDir;


pub struct DirGrowthReporter {
    total_size : u64,                   // total file size of all directories scanned
    buckets : BTreeMap<String, u64>,    // map of year-month -> total file size during that month
}

impl DirGrowthReporter {

    pub fn new() -> Self {
        return DirGrowthReporter {
            total_size: 0,
            buckets: BTreeMap::new(),
        };
    }

    pub fn load_file_info(&mut self, path: &str) {
        // use filter_map to ignore access errors
        for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
            if file.file_type().is_file() {
                let file_time: DateTime<Utc> = file.metadata().unwrap().modified().unwrap().into();
                let file_size: u64 = file.metadata().unwrap().len();

                let bucket_key = format!("{}-{:02}", file_time.year(), file_time.month());
                let bucket_size = self.buckets.entry(bucket_key).or_insert(0);
                *bucket_size += file_size;
                self.total_size += file_size;
            }
        }
    }

    pub fn console_output(&self) {
        let console_size = termsize::get().unwrap();

        let col_max : u16 = console_size.cols / 2; // half the console width seems reasonable
        let bytes_per_col = self.total_size / u64::try_from(col_max).unwrap();
        let mut size_to_date : u64 = 0;
        for (key, value) in self.buckets.iter() {
            size_to_date += *value;
            let width : u16 = u16::try_from(size_to_date / bytes_per_col).unwrap();
            let mut graph_line : String = String::new();
            graph_line.push_str(&String::from_utf8(vec![b'='; usize::try_from(width).unwrap()]).unwrap());
            graph_line.push_str(&String::from_utf8(vec![b' '; usize::try_from(col_max - width).unwrap()]).unwrap());
            graph_line.push_str("  ");
            graph_line.push_str(&human_bytes(size_to_date as f64));
            println!("{}: {}", key, graph_line);
        }

    }

}

