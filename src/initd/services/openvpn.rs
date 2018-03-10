use std::default::Default;
use super::ServiceConfig;


#[derive(Debug)]
pub struct OpenVPN {
    executable: &'static str
}

impl ServiceConfig for OpenVPN {
    fn executable(&self) -> String {
        self.executable.to_string()
    }

    fn name(&self) -> &'static str {
        "OpenVPN"
    }
}

impl Default for OpenVPN {
    fn default() -> Self {
        Self { executable: "./1.sh" }
    }
}

impl OpenVPN {

}
