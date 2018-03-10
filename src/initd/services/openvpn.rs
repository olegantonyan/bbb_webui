use std::default::Default;
use super::ServiceConfig;


#[derive(Debug)]
pub struct OpenVPN {
}

impl ServiceConfig for OpenVPN {
    fn start_command(&self) -> String {
        "./1.sh".to_string()
    }
}

impl Default for OpenVPN {
    fn default() -> Self {
        Self { }
    }
}

impl OpenVPN {

}
