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
    pub fn execute(&self) -> Result<String, String> {
        let result = Command::new(self.command).output();
        match result {
            Ok(_) => Ok("rebooting now...".to_string()),
            Err(error) => Err(error.to_string())
        }
    }
}
