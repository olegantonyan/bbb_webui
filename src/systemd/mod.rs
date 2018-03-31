use std::process::Command as Command;


#[derive(Debug)]
pub struct Systemd {
}

impl Systemd {
    pub fn start(service_name: &String) -> Result<String, String> {
        Self::daemon_reload();
        let mut command = Command::new("systemctl");
        command.args(&["start", service_name.as_str()]);
        Self::execute(command)
    }

    pub fn stop(service_name: &String) -> Result<String, String> {
        Self::daemon_reload();
        let mut command = Command::new("systemctl");
        command.args(&["stop", service_name.as_str()]);
        Self::execute(command)
    }

    pub fn restart(service_name: &String) -> Result<String, String> {
        Self::daemon_reload();
        let s = Self::stop(service_name);
        if s.is_err() {
            return s;
        }
        Self::start(service_name)
    }

    pub fn status(service_name: &String) -> Result<String, String> {
        let mut command = Command::new("systemctl");
        command.args(&["status", service_name.as_str(), "--no-pager", "-n", "0"]);
        Self::execute(command)
    }

    pub fn journal(service_name: &String, max_lines: u32) -> Result<String, String> {
        let mut command = Command::new("journalctl");
        command.args(&["-u", service_name.as_str(), "--no-pager", "-n", max_lines.to_string().as_str()]);
        Self::execute(command)
    }

    pub fn daemon_reload() {
        let mut command = Command::new("systemctl");
        command.args(&["daemon-reload"]);
        match Self::execute(command) {
            Ok(_) => (),
            Err(err) => eprintln!("Error excecuting daemon-reload: {:?}", err)
        }
    }

    fn execute(mut command: Command) -> Result<String, String> {
        let result = command.output();
        match result {
            Ok(output) => {
                if output.status.success() {
                    Ok(String::from_utf8(output.stdout).unwrap_or(String::new()))
                } else {
                    Err(String::from_utf8(output.stderr).unwrap_or(String::new()))
                }
            },
            Err(error) => Err(error.to_string())
        }
    }
}
