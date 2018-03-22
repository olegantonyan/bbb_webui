use std::default::Default;

use ::systemd::Systemd as Systemd;


#[derive(Debug)]
pub struct OpenVPN {
    service_name: &'static str
}

impl Default for OpenVPN {
    fn default() -> Self {
        Self { service_name: "tgvpn" }
    }
}

impl OpenVPN {
    pub fn start(&self) -> Result<String, String> {
        Systemd::start(self.service_name)
    }

    pub fn stop(&self) -> Result<String, String> {
        Systemd::stop(self.service_name)
    }

    pub fn restart(&self) -> Result<String, String> {
        Systemd::restart(self.service_name)
    }

    pub fn status(&self) -> Result<String, String> {
        Systemd::status(self.service_name)
    }

    pub fn logs(&self) -> Result<String, String> {
        Systemd::journal(self.service_name, 128)
    }

    pub fn change_config(&self, filename: String) -> Result<String, String> {
        Ok("TODO".to_string())
    }

    pub fn available_configs(&self) -> Vec<String> {
        vec!["one".to_string(), "two".to_string(), "ololol".to_string()]
    }

    pub fn current_config(&self) -> String {
        "ololol".to_string()
    }
}
