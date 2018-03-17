use std::process::Command;
use std::default::Default;


#[derive(Debug)]
pub struct Reboot {
    command: &'static str
}

impl Default for Reboot {
    fn default() -> Self {
        Self { command: "reboot" }
    }
}

impl Reboot {
    pub fn execute(&self) -> String {
        let result = Command::new(self.command).output();
        match result {
            Ok(_) => "rebooting now...".to_string(),
            Err(error) => error.to_string()
        }
    }
}
