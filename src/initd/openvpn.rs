use std::default::Default;
use super::Processable;


#[derive(Debug)]
pub struct OpenVPN {
}

impl Processable for OpenVPN {
    fn command(&self) -> String {
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
