use std::default::Default;


#[derive(Debug)]
pub struct OpenVPN {
}

impl Default for OpenVPN {
    fn default() -> Self {
        Self { }
    }
}

impl OpenVPN {
    pub fn start(&self) {
    }

    pub fn stop(&self) {
    }

    pub fn restart(&self) {
    }

    pub fn status(&self) {
    }

    pub fn logs(&self) {
    }
}
