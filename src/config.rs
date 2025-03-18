use anyhow::Result;
use serde::Deserialize;
use serde_json::from_reader;
use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub api_key: String,
    pub active_profile: String,
    pub profiles: HashMap<String, Profile>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Profile {
    pub model_name: String,
    pub chars: usize,
    pub system_prompt: Vec<String>,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let f = File::open(path)?;
        let r = BufReader::new(f);
        let mut config: Config = from_reader(r)?;

        config.render();

        Ok(config)
    }

    fn render(&mut self) {
        for (_, val) in self.profiles.iter_mut() {
            for line in &mut val.system_prompt {
                *line = line.replace("{chars}", val.chars.to_string().as_str());
            }
        }
    }
}
