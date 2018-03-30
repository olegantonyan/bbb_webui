use std::fs;

use ::systemd::Systemd as Systemd;

#[derive(Debug)]
pub struct Config {
    pub dir: String,
    pub current_config_symlink_name: String,
    pub service_name: String,
}

#[derive(Debug)]
pub struct OpenVPN {
    config: Config
}

impl OpenVPN {
    pub fn new(c: &Config) -> Self {
        Self {
            config: Config { dir: c.dir.clone(), current_config_symlink_name: c.current_config_symlink_name.clone(), service_name: c.service_name.clone() }
        }
    }

    pub fn start(&self) -> Result<String, String> {
        Systemd::start(&self.config.service_name)
    }

    pub fn stop(&self) -> Result<String, String> {
        Systemd::stop(&self.config.service_name)
    }

    pub fn restart(&self) -> Result<String, String> {
        Systemd::restart(&self.config.service_name)
    }

    pub fn status(&self) -> Result<String, String> {
        Systemd::status(&self.config.service_name)
    }

    pub fn logs(&self) -> Result<String, String> {
        Systemd::journal(&self.config.service_name, 128)
    }

    pub fn change_config(&self, filename: String) -> Result<String, String> {
        Ok("TODO".to_string())
    }

    pub fn available_configs(&self) -> Vec<String> {
        let paths = fs::read_dir("./").unwrap();

        for path in paths {
            println!("Name: {:?}", path.unwrap().path())
        }
        vec!["one".to_string(), "two".to_string(), "ololol".to_string()]
    }

    pub fn current_config(&self) -> String {
        "ololol".to_string()
    }
}
