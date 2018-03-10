use std::process::Command;
use std::default::Default;


#[derive(Debug)]
pub struct Reboot {
    command: &'static str
}

impl Default for Reboot {
    fn default() -> Self {
        Reboot { command: "reboot" }
    }
}

impl Reboot {
    pub fn execute(&self) {
        Command::new(self.command).spawn().expect(format!("failed to execute `{}`", self.command).as_str());
    }
}
