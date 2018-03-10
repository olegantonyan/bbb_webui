pub mod process;
pub mod services;

use self::services::ServiceConfig;
use self::process::Process;

use std::time::Duration;
use std::thread;
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

    pub fn start_service<T: ServiceConfig>(&mut self, config: T) -> Result<&Process, &'static str> {
        let process = Process::new();
        let service_name = config.name();
        if self.processes.contains_key(service_name) {
            return Err("service already exists");
        }

        process.start(config);
        self.processes.insert(service_name, process);

        //thread::sleep(Duration::from_millis(1000));
        //p.term(t);


        //println!("{:?}", self.process(service_name).unwrap().state());
        Ok(&self.process(service_name).unwrap())
    }

    pub fn stop_service<T: ServiceConfig>(&mut self, config: T) {
        let service_name = config.name();
        let mut remove = false;
        {   // fucking hate this "as mutable because it is also borrowed as immutable"
            let process = self.process(service_name);
            if process.is_some() {
                process.unwrap().term();
                remove = true;
            }
        }
        if remove {
            self.processes.remove(service_name);
        }
    }

    pub fn restrart_service<T: ServiceConfig>(&mut self, config: T) {
        self.stop_service(config);
        self.start_service(config);
    }
}
