use chrono::{DateTime, Local};
use std::collections::HashMap;
use std::fs::{OpenOptions};
use std::fs;
use std::io::Write;

#[derive(Default)]
pub struct SessionMetrics{
    //pub name: String,
    pub start: DateTime<Local>,
    pub duration_min: u32,
    pub window_switches: u32,
    pub distractor_hits: u32,
    pub total_processes: u32,
    pub idle_seconds: u32,
    pub charms_earned: u32,
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
            //self.name,
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
pub fn calculate_charms(metrics: &SessionMetrics) -> u32 {
    let base = metrics.duration_min / 5;
    let penalty = metrics.distractor_hits / 3;
    //println!("Base and Penalty: {} {}", base, penalty );
    base.saturating_sub(penalty)
}

//-----Load charms from JSON file-----
pub fn load_total_charms(user: &str) -> u32{
    let path = "output/charms.json";
    if let Ok(contents) = fs::read_to_string(path){
        if let Ok(map) = serde_json::from_str::<HashMap<String, u32>>(&contents){
            return *map.get(user).unwrap_or(&0);
        }
    }
    0
}

//---Saving charms----
pub fn save_total_charms(user: &str, charms: u32){
    let path = "output/charms.json";
    let mut map = if let Ok(contents) = fs::read_to_string(path){
        serde_json::from_str::<HashMap<String,u32>>(&contents).unwrap_or_default()
    } else {
        HashMap::new()
    };

    map.insert(user.to_string(), charms);

    if let Ok(json) = serde_json::to_string_pretty(&map){
        fs::write(path,json).unwrap();
    }

}