use std::fs;
use std::process::Command as Command;

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
        let r = RwFs::new();
        let result = r.rw();
        if result.is_err() {
            return result;
        }

        let mut command = Command::new("rm");
        command.args(&[self.path_to_current_config().as_str()]);
        let result = self.execute(command);
        if result.is_err() {
            return result;
        }

        let mut command = Command::new("ln");
        command.args(&["-s", format!("{}/{}", self.config.dir, filename).as_str(), self.path_to_current_config().as_str()]);
        let result = self.execute(command);
        if result.is_err() {
            return result;
        }

        let result = r.ro();
        if result.is_err() {
            return result;
        }
        self.restart()
    }

    pub fn available_configs(&self) -> Vec<String> {
        let paths = fs::read_dir(&self.config.dir).unwrap();
        let mut configs = Vec::new();
        configs.push(String::new());
        for path in paths {
            let p = path.unwrap().path().file_name().unwrap().to_str().unwrap().to_string();
            if p.ends_with(self.config.vpn_config_file_suffix.as_str()) {
                configs.push(p);
            }
        }
        configs
    }

    pub fn current_config(&self) -> String {
        let path = fs::read_link(&self.path_to_current_config());
        match path {
            Ok(fname) => fname.file_name().unwrap().to_str().unwrap().to_string(),
            Err(_) => String::new()
        }
    }

    fn execute(&self, mut command: Command) -> Result<String, String> {
        let result = command.output();
        match result {
            Ok(output) => {
                if output.status.success() {
                    Ok(String::from_utf8(output.stdout).unwrap_or(String::new()))
                } else {
                    Err(String::from_utf8(output.stderr).unwrap_or(String::new()))
                }
            },
            Err(error) => Err(error.to_string())
        }
    }

    fn path_to_current_config(&self) -> String {
        format!("{}/{}", self.config.dir, self.config.current_config_symlink_name)
    }
}
