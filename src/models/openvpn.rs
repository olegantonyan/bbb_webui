use std::fs;

use ::systemd::Systemd as Systemd;
use ::rwfs::RwFs as RwFs;

#[derive(Debug)]
pub struct Config {
    pub dir: String,
    pub current_config_symlink_name: String,
    pub service_name: String,
    pub vpn_config_file_suffix: String
}

#[derive(Debug)]
pub struct OpenVPN {
    config: Config
}

impl OpenVPN {
    pub fn new(c: &Config) -> Self {
        Self {
            config: Config {
                dir: c.dir.clone(),
                current_config_symlink_name: c.current_config_symlink_name.clone(),
                service_name: c.service_name.clone(),
                vpn_config_file_suffix: c.vpn_config_file_suffix.clone() }
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
        let paths = fs::read_dir(&self.config.dir).unwrap();
        let mut configs = Vec::new();
        for path in paths {
            let p = path.unwrap().path().to_str().unwrap().to_string();
            if p.ends_with(self.config.vpn_config_file_suffix.as_str()) {
                let i = p.replace(&self.config.dir, "").replace("/", "");
                configs.push(i);
            }
        }
        configs
    }

    pub fn current_config(&self) -> String {
        "ololol".to_string()
    }
}
