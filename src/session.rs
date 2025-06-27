use chrono::{DateTime, Local};
use std::fs::{OpenOptions};
use std::io::Write;

#[derive(Default)]
pub struct SessionMetrics{
    pub start: DateTime<Local>,
    pub duration_min: u32,
    pub window_switches: u32,
    pub distractor_hits: u32,
    pub total_processes: u32,
    pub idle_seconds: u32,
}

impl SessionMetrics{
    pub fn save_csv(&self, path: &str){
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .expect("cannot open session log");

        let write_header = f.metadata().unwrap().len() == 0;
        if write_header {
            writeln!(f, "start,duration_min,window_switches,distractor_hits,total_process,idle_seconds").unwrap();
        }
        let line = format!(
            "{},{},{},{},{},{}\n",
            self.start.format("%Y-%m-%d %H:%M:%S"),
            self.duration_min,
            self.window_switches,
            self.distractor_hits,
            self.total_processes,
            self.idle_seconds
        );
        f.write_all(line.as_bytes()).unwrap();
    }
}