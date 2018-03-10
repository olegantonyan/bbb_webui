pub mod process;
pub mod services;

use self::services::ServiceConfig;
use self::process::Process;

use std::time::Duration;
use std::thread;
use std::vec::Vec;

pub struct InitD {
    handles: Vec<ProcessHandle>
}

impl Default for InitD {
    fn default() -> Self {
        Self { handles: Vec::new() }
    }
}

impl InitD {
    pub fn start_process<T: ServiceConfig>(&mut self, config: T) {
        let p = Process::new();
        let t = p.start(config);

        let h = ProcessHandle {
            thread: t,
            process: p
        };
        self.handles.push(h);



        thread::sleep(Duration::from_millis(1000));
        //p.term(t);


        println!("{:?}", self.handles[0].process.state());
    }
}

struct ProcessHandle {
    thread: thread::JoinHandle<()>,
    process: Process
}
