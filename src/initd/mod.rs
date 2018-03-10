pub mod openvpn;

pub use initd::openvpn::OpenVPN;

use std::default::Default;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::thread;
use std::vec::Vec;
use std::sync::{Mutex, Arc};
use std::marker::Sync;


pub trait Processable {
    fn command(&self) -> String;
}

pub struct InitD {
}

impl Default for InitD {
    fn default() -> Self {
        Self {}
    }
}

//impl<T: Processable> InitD {
//    pub fn start_process(&self, process_config: T) {
//        Process::new(process_config);
//    }
//}

#[derive(Debug, Clone)]
pub struct Process {
    pub output: Arc<Mutex<ProcessOutput>>
}

impl Process {
    pub fn new() -> Self {
        Self { output: Arc::new(Mutex::new(ProcessOutput::default())) }
    }

    pub fn start(&self) -> thread::JoinHandle<()> {
        let self_clone = self.clone();

        thread::spawn(move || {
            let mut child = Command::new("./1.sh").stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();

            {   // handle stderr
                let procout = Arc::clone(&self_clone.output);
                let err = BufReader::new(child.stderr.take().unwrap());
                thread::spawn(move || {
                    for line in err.lines() {
                        let mut pout = procout.lock().unwrap();
                        pout.append_line(line.unwrap(), ProcessOutputType::STDERR)
                    }
                });
            }

            {   // handle stdout
                let procout = Arc::clone(&self_clone.output);
                let out = BufReader::new(child.stdout.take().unwrap());
                thread::spawn(move || {
                    for line in out.lines() {
                        let mut pout = procout.lock().unwrap();
                        pout.append_line(line.unwrap(), ProcessOutputType::STDOUT)
                    }
                });
            }

            let status = child.wait().unwrap();
            //println!("{}", status);
            //println!("{:?}", &self_clone.output);

        })
    }
}

#[derive(Debug)]
pub struct ProcessOutput {
    stderr: Vec<String>,
    stdout: Vec<String>,
    cap: usize
}

impl ProcessOutput {
    pub fn append_line(&mut self, data: String, out_type: ProcessOutputType) {
        match out_type {
            ProcessOutputType::STDERR => {
                if self.stderr.len() > self.cap {
                    self.stderr.pop();
                }
                //println!("err: {}", data);
                self.stderr.push(data)
            },
            ProcessOutputType::STDOUT => {
                if self.stdout.len() > self.cap {
                    self.stdout.pop();
                }
                //println!("out: {}", data);
                self.stdout.push(data)
            }
        }
    }
}

impl Default for ProcessOutput {
    fn default() -> Self {
        Self { stderr: Vec::new(), stdout: Vec::new(), cap: 32768 }
    }
}

enum ProcessOutputType {
    STDERR,
    STDOUT
}
