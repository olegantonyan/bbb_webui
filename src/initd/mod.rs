pub mod openvpn;

pub use initd::openvpn::OpenVPN;

use std::default::Default;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::thread;
use std::vec::Vec;
use std::sync::{Mutex, Arc};


pub trait Processable {
    fn command(&self) -> String;
}

pub struct InitD<T: Processable> {
    processes: Vec<T>
}

impl<T: Processable> Default for InitD<T> {
    fn default() -> Self {
        Self { processes: Vec::new() }
    }
}

impl<T: Processable> InitD<T>  {
    pub fn start_process(&self, process: T) {
        self.execute(process.command());
    }

    fn execute(&self, executable: String) -> thread::JoinHandle<()> {
        thread::spawn(|| {
            let mut child = Command::new(executable).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
            
            let procout = Arc::new(Mutex::new(ProcessOutput::default()));

            {   // handle stderr
                let procout = Arc::clone(&procout);
                let err = BufReader::new(child.stderr.take().unwrap());
                thread::spawn(move || {
                    for line in err.lines() {
                        let mut pout = procout.lock().unwrap();
                        pout.append_line(line.unwrap(), ProcessOutputType::STDERR)
                    }
                });
            }

            {   // handle stdout
                let procout = Arc::clone(&procout);
                let out = BufReader::new(child.stdout.take().unwrap());
                thread::spawn(move || {
                    for line in out.lines() {
                        let mut pout = procout.lock().unwrap();
                        pout.append_line(line.unwrap(), ProcessOutputType::STDOUT)
                    }
                });
            }

            let status = child.wait().unwrap();
            println!("{}", status);
            println!("{:?}", procout);

        })
    }
}

#[derive(Debug)]
struct ProcessOutput {
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
