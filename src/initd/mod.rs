pub mod process;
pub mod services;

use self::services::ServiceConfig;
use self::process::Process;

use std::collections::HashMap;

pub struct InitD {
    processes: HashMap<&'static str, Process>
}

impl Default for InitD {
    fn default() -> Self {
        Self { processes: HashMap::new() }
    }
}

impl InitD {
    pub fn process(&self, service_name: &'static str) -> Option<&Process> {
        if self.processes.contains_key(service_name) {
            Some(&self.processes[service_name])
        } else {
            None
        }
    }

    pub fn start_service<T: ServiceConfig>(&mut self, config: &T) -> Result<&Process, &'static str> {
        let process = Process::new();
        let service_name = config.name();
        match self.process(service_name) {
            Some(ref process) => {
                if process.state().is_running() {
                    return Err("service already exists");
                }
            },
            None => ()
        }
        process.start(config);
        self.processes.insert(service_name, process);
        Ok(&self.process(service_name).unwrap())
    }

    pub fn stop_service<T: ServiceConfig>(&mut self, config: &T) {
        let service_name = config.name();
        if self.stop_process(service_name) {
            self.processes.remove(service_name);
        }
    }

    pub fn restrart_service<T: ServiceConfig>(&mut self, config: &T) {
        self.stop_service(config);
        self.start_service(config);
    }

    fn stop_process(&self, service_name: &'static str) -> bool {
        let process = self.process(service_name);
        if process.is_some() {
            process.unwrap().term();
            true
        } else {
            false
        }
    }
}
