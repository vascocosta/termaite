use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_string_pretty};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, ErrorKind, Write},
    path::Path,
    process,
};

pub(crate) const CONF_FILE: &str = ".termaite.json";

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Config {
    pub api_key: String,
    pub active_profile: String,
    pub profiles: HashMap<String, Profile>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Profile {
    pub model_name: String,
    pub chars: usize,
    pub system_prompt: Vec<String>,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let f = match File::open(path) {
            Ok(f) => f,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => {
                    eprintln!("Could not find a configuration file.");
                    eprint!("Creating a default configuation file...");

                    let conf_str = to_string_pretty(&Config::default())?;
                    let conf_path = dirs::home_dir()
                        .context("Could not find home dir.")?
                        .join(CONF_FILE);
                    let mut f =
                        File::create(&conf_path).context("Could not create configuration file.")?;

                    f.write_all(conf_str.as_bytes())?;

                    eprintln!("OK");
                    eprintln!(
                        "You can find your configuration here: {}",
                        conf_path.to_string_lossy()
                    );
                    eprintln!("Please add your API key to the configuration file.");
                    eprintln!("Then try to run the program again.");

                    process::exit(1);
                }
                _ => {
                    eprintln!("Unknown error trying to open configuration.");
                    process::exit(1);
                }
            },
        };

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

impl Default for Config {
    fn default() -> Self {
        let mut profiles: HashMap<String, Profile> = HashMap::new();
        profiles.insert(String::from("default"), Profile::default());

        Self {
            api_key: String::from("YOUR_API_KEY"),
            active_profile: String::from("default"),
            profiles,
        }
    }
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            model_name: String::from("gemini-2.0-flash"),
            chars: 4000,
            system_prompt: Vec::default(),
        }
    }
}
