use rustc_serialize::json;
use std::sync::RwLock;
use std::io::prelude::*;
use std::fs::File;

use lib::config::SiteConfig;
use lib::error;

const CONFIG_FOLDER: &'static str = "private/config/";

lazy_static! {
    static ref CONFIG: RwLock<Config> = RwLock::new(Config::default());
}

#[derive(Default, Debug, Clone)]
pub struct Config {
    site_config: SiteConfig,
}

impl Config {
    pub fn init_config(new_site: bool) {
        match CONFIG.write() {
            Ok(mut config) => {
                config.site_config = Config::read_config().unwrap_or_else(|e| {
                    match e {
                        error::LibError::Io(_) => {
                            println!("Failed to load site configurations: {:?}", e);
                            if new_site {
                                println!("Setting a new site, falling back to default configuration.");
                                SiteConfig::default()
                            } else {
                                println!("If you are trying to set up a new site use the \"--new\" cli \
                                          argument.");
                                panic!();
                            }
                        }
                        _ => {
                            panic!("Failed to load site configurations: {:?}", e);
                        }
                    }
                });
                println!("Configurations loaded");
            }
            Err(e) => {
                panic!("Error getting a write lock on CONFIG: {:?}", e);
            }
        }
    }

    pub fn read_config() -> Result<SiteConfig, error::LibError> {
        Config::load_json(format!("{}{}", CONFIG_FOLDER, "site_data.json").as_str())
    }

    #[allow(dead_code)]
    pub fn write_config() -> Result<(), error::LibError> {
        match CONFIG.read() {
            Ok(config) => {
                Config::save_json(&config.site_config,
                                  format!("{}{}", CONFIG_FOLDER, "site_data.json").as_str())
            }
            Err(e) => {
                println!("Error getting a read lock on Config: {:?}", e);
                Err(error::LibError::PoisonedLock)
            }
        }
    }

    fn load_json(file_name: &str) -> Result<SiteConfig, error::LibError> {
        let mut file = try!(File::open(file_name));
        let mut content = String::new();
        let _ = file.read_to_string(&mut content);
        let new_config = try!(json::decode(content.as_str()));
        Ok(new_config)
    }

    #[allow(dead_code)]
    fn save_json(site_data: &SiteConfig, file_name: &str) -> Result<(), error::LibError> {
        let mut file = try!(File::create(file_name));
        let content = try!(json::encode(site_data));
        let _ = file.write_all(content.as_bytes());
        Ok(())
    }

    pub fn get_site_config() -> Result<SiteConfig, error::LibError> {
        match CONFIG.read() {
            Ok(config) => Ok(config.site_config.clone()),
            Err(e) => {
                println!("Error getting a read lock on Config: {:?}", e);
                Err(error::LibError::PoisonedLock)
            }
        }
    }

    #[allow(dead_code)]
    pub fn set_site_data(site_data: SiteConfig) -> Result<(), error::LibError> {
        match CONFIG.write() {
            Ok(mut config) => {
                config.site_config = site_data;
                Ok(())
            }
            Err(e) => {
                println!("Error getting a write lock on CONFIG: {:?}", e);
                Err(error::LibError::PoisonedLock)
            }
        }
    }
}
