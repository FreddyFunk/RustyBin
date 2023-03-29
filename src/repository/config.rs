use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Error, ErrorKind};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Config {
    #[serde(default = "default_server_port")]
    pub server_port: u16,
    #[serde(default = "default_worker_count")]
    pub worker_count: usize,
    #[serde(default = "default_data_path")]
    pub data_path: String,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default = "default_allow_overwrite")]
    pub allow_overwrite: bool,
    #[serde(default = "default_allow_delete")]
    pub allow_delete: bool,
    #[serde(default = "default_memory_cache_size_per_instance")]
    pub memory_cache_size_per_instance: u64,
}

fn default_server_port() -> u16 {
    8080
}

fn default_data_path() -> String {
    "./data/".to_string()
}

fn default_log_level() -> String {
    "DEBUG".to_string()
}

fn default_allow_overwrite() -> bool {
    false
}

fn default_allow_delete() -> bool {
    false
}

fn default_memory_cache_size_per_instance() -> u64 {
    16
}

fn default_worker_count() -> usize {
    num_cpus::get().min(1)
}

fn open_file(filename: &str) -> Result<File, Error> {
    let file = File::open(filename)?;
    Ok(file)
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let filename = "config.yml";
    let file = open_file(filename);
    let config: Config;
    match file {
        Ok(_) => {    
            let reader = std::io::BufReader::new(file.unwrap());
            config = serde_yaml::from_reader(reader)?;
        }
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            // Log a warning when no config file is not found
            eprintln!("Warning: config file not found. Using default values.");
            config = serde_yaml::from_str(&"")?;
            
        }
        Err(error) => {
            // Return an error for any other io error
            return Err(Box::new(error));
        }
    }

    std::env::set_var("RUST_LOG", config.log_level.clone());
    std::env::set_var("RUST_BACKTRACE", "1");

    let conf = serde_yaml::to_string(&config);
    print!("{}", conf.unwrap());

    return Ok(config);
}
