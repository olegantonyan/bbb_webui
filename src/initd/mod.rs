pub mod process;
pub mod services;

use self::services::ServiceConfig;
use self::process::Process;

use std::time::Duration;
use std::thread;

pub struct InitD {
}

impl Default for InitD {
    fn default() -> Self {
        Self {}
    }
}

impl InitD {
    pub fn start_process<T: ServiceConfig>(&self, config: T) {
        let p = Process::new();
        let t = p.start(config);

        let h = ProcessHandle {
            thread: t,
            process: p
        }



        //thread::sleep(Duration::from_millis(1000));
        //p.term(t);


        println!("{:?}", h.process.p.state());
    }
}

struct ProcessHandle {
    thread: thread::JoinHandle<()>,
    process: Process
}
