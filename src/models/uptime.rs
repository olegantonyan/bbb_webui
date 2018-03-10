use std::process::Command;
use std::default::Default;


#[derive(Debug)]
pub struct Uptime {
    command: &'static str
}

impl Default for Uptime {
    fn default() -> Self {
        Uptime { command: "uptime" }
    }
}

impl Uptime {
    pub fn execute(&self) -> String {
        let uptime_stdout = Command::new(self.command).output().expect(format!("failed to execute `{}`", self.command).as_str()).stdout;
        String::from_utf8(uptime_stdout).unwrap()
    }
}
