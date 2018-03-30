use std::process::Command as Command;


#[derive(Debug)]
pub struct RwFs {
    fs: String
}

impl RwFs {
    pub fn new() -> Self {
        Self { fs: "/".to_owned() }
    }

    pub fn rw(&self) -> Result<String, String> {
        self.remount("rw")
    }

    pub fn ro(&self) -> Result<String, String> {
        self.remount("ro")
    }

    fn remount(&self, mode: &'static str) -> Result<String, String> {
        let mut command = Command::new("mount");
        command.args(&["-o", format!("remount,{}", mode).as_str(), self.fs.as_str()]);
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
