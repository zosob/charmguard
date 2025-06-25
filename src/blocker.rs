use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct BlockList{
    pub apps: Vec<String>,
}

impl BlockList{
    pub fn load(path: &str) -> Self{
        let raw = fs::read_to_string(path)
            .unwrap_or_else(|_| panic!("Could not read {path}"));
        serde_yaml::from_str(&raw)
            .unwrap_or_else(|_| panic!("Invalid YAML in {path}"))
    }

    pub fn is_distracting(&self, proc_name: &str) -> bool {
        let name_l = proc_name.to_lowercase();
        self.apps.iter().any(|pat| name_l.contains(pat))
    }
}