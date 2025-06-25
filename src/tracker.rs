use sysinfo::{System, ProcessExt, SystemExt};
use chrono::Local;

#[derive(Debug)]
pub struct ProcessSnapshot{
    pub name: String,
    pub cpu: f32,
    pub memory: u64,
    pub timestamp: String,
}

pub fn capture_snapshot() -> Vec<ProcessSnapshot>{
    let mut sys = System::new_all();
    sys.refresh_all();

    let ts = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    sys.processes()
        .values()
        .map(|p| ProcessSnapshot {
            name: p.name().to_owned(),
            cpu: p.cpu_usage(),
            memory: p.memory(),
            timestamp: ts.clone(),
        })
        .collect()
}