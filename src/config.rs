use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use config::Config;
use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::error::Error::Generic;
use crate::prelude::*;

const CONFIG_FILE: &str = "profiles.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct RPCConfig {
    // Map of profile names to RPCProfile
    pub profiles: HashMap<String, RPCProfile>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RPCProfile {
    pub client_id: String,
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
    pub state: Option<String>,
    pub details: Option<String>,
}

impl RPCConfig {
    pub fn new() -> Self {
        let config = match Self::try_load() {
            Ok(c) => c,
            Err(e) => {
                Self::handle_error(&e);
                return Self::new();
            }
        };

        let config = Self::read_file(config).unwrap_or_else(|e| {
            error!("Error reading profiles: {}", e);
            std::process::exit(99);
        });

        config
    }


    fn try_load() -> Result<Config> {
        let current_dir = Self::get_current_dir()?;
        let config = Config::builder()
            .add_source(config::File::from(current_dir.join(CONFIG_FILE)))
            .build();

        config.map_err(|e| Error::ConfigError(e))
    }
    fn handle_error(e: &Error) {
        let match_string = "not found";
        if !e.to_string().contains(match_string) {
            error!("Critical Error, loading profiles: {}", e.to_string());
            std::process::exit(99);
        }
        info!("I didn't find any profiles, creating a new empty one!");
        if let Err(e) = Self::create_config_file() {
            error!("Error creating new profile: {}", e.to_string());
            std::process::exit(99);
        }
    }
    fn create_config_file() -> Result<()> {
        let dir = Self::get_current_dir()?.join(CONFIG_FILE);
        let path = dir.as_path();

        if path.exists(){
            info!("Profile file already exists, do you want to overwrite it with a new empty one? (y/n)");
            if !Self::can_i_proceed() {
                return Ok(());
            }
        }
        let mut file = File::create(path)?;
        let contents = toml::to_string_pretty(&RPCConfig::default())?;
        file.write_all(contents.as_bytes())?;

        log::info!("Created new profile file: {}", path.display());

        Ok(())
    }
    fn get_current_dir() -> Result<PathBuf> {
        let mut dir = std::env::current_exe()?;
        dir.pop();
        Ok(dir)
    }

    fn read_file(config: Config) -> Result<Self> {
        let de_settings: Self = match config.try_deserialize().map_err(|e| Error::ConfigError(e)){
            Ok(s) => s,
            Err(e) => {
                error!("Error deserializing profiles: {}", e);

                if let Some(fields) = Self::is_it_missing_field(&e) {
                    error!("Missing field: {}", fields);
                    info!("Would you like to create a new profile file? (y/n)");
                    if !Self::can_i_proceed() {
                        error!("You chose not to create a new profile file, aborting...");
                        std::process::exit(99);
                    }

                    if let Err(e) = Self::create_config_file() {
                        error!("Error creating new profile: {}", e.to_string());
                        return Err(Generic(e.to_string()));
                    }

                    return Ok(Self::new());
                }

                return Err(Generic(e.to_string()))
            }
        };

        log::trace!("Deserialized profiles: {:?}", de_settings);

        Ok(de_settings)
    }
    fn is_it_missing_field(error: &Error) -> Option<String> {
        let match_string = "missing field";
        if !error.to_string().contains(match_string) {
            return None;
        }

        let fields = error.to_string().replace("missing field ", "");
        Some(fields)
    }
    fn can_i_proceed() -> bool {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap_or_else(|e| {
            error!("Fatal Error reading input: {}", e);
            std::process::exit(99);
        });

        input.trim() != "y"
    }
}
impl Default for RPCConfig {
    fn default() -> Self {
        let demo_profile = RPCProfile {
            client_id: "1234567890".to_string(),
            large_image: Some("large_image".to_string()),
            large_text: Some("large_text".to_string()),
            small_image: Some("small_image".to_string()),
            small_text: Some("small_text".to_string()),
            state: Some("state".to_string()),
            details: Some("details".to_string()),
        };

        let mut profiles = HashMap::new();
        profiles.insert("demo".to_string(), demo_profile);

        Self {
            profiles
        }
    }
}