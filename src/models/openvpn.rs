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
        Systemd::journal(self.service_name) // TODO pass number of lines
    }
}
