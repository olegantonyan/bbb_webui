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
    pub fn start(&self) -> String {
        match Systemd::start(self.service_name) {
            Ok(stdout) => stdout,
            Err(stderr) => stderr
        }
    }

    pub fn stop(&self) -> String {
        match Systemd::stop(self.service_name) {
            Ok(stdout) => stdout,
            Err(stderr) => stderr
        }
    }

    pub fn restart(&self) -> String {
        match Systemd::restart(self.service_name) {
            Ok(stdout) => stdout,
            Err(stderr) => stderr
        }
    }

    pub fn status(&self) -> String {
        match Systemd::status(self.service_name) {
            Ok(stdout) => stdout,
            Err(stderr) => stderr
        }
    }

    pub fn logs(&self) -> String {
        match Systemd::journal(self.service_name) {
            Ok(stdout) => stdout,
            Err(stderr) => stderr
        }
    }
}
