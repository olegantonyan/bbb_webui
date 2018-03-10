use std::default::Default;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::thread;
use std::vec::Vec;
use std::sync::{Mutex, Arc};
use std::marker::Sync;


#[derive(Debug, Clone)]
pub struct Process {
    state: Arc<Mutex<ProcessState>>
}

impl Process {
    pub fn new() -> Self {
        Self { state: Arc::new(Mutex::new(ProcessState::default())) }
    }

    pub fn state(&self) -> ProcessState {
        let procout = Arc::clone(&self.state);
        let pout = procout.lock().unwrap();
        (*pout).clone()
    }

    pub fn start(&self) -> thread::JoinHandle<()> {
        let self_clone = self.clone();

        thread::spawn(move || {
            let mut child = Command::new("./1.sh").stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();

            /*{   // handle stderr
                let procout = Arc::clone(&self_clone.state);
                let err = BufReader::new(child.stderr.take().unwrap());
                thread::spawn(move || {
                    for line in err.lines() {
                        let mut pout = procout.lock().unwrap();
                        pout.append_line(line.unwrap(), ProcessOutputType::STDERR)
                    }
                });
            }*/
            self.setup_stderr();

            {   // handle stdout
                let procout = Arc::clone(&self_clone.state);
                let out = BufReader::new(child.stdout.take().unwrap());
                thread::spawn(move || {
                    for line in out.lines() {
                        let mut pout = procout.lock().unwrap();
                        pout.append_line(line.unwrap(), ProcessOutputType::STDOUT)
                    }
                });
            }

            {
                let procout = Arc::clone(&self_clone.state);
                let mut pout = procout.lock().unwrap();
                pout.set_pid(child.id());
            }

            let status = child.wait().unwrap();
            //println!("{}", status);
            //println!("{:?}", &self_clone.output);

        })
    }

    fn setup_stderr(&self) {
        let procout = Arc::clone(&self.clone().state);
        let err = BufReader::new(child.stderr.take().unwrap());
        thread::spawn(move || {
            for line in err.lines() {
                let mut pout = procout.lock().unwrap();
                pout.append_line(line.unwrap(), ProcessOutputType::STDERR)
            }
        });
    }
}

#[derive(Debug, Clone)]
pub struct ProcessState {
    stderr: Vec<String>,
    stdout: Vec<String>,
    pid: u32,
    output_max_lines: usize
}

impl ProcessState {
    pub fn append_line(&mut self, data: String, out_type: ProcessOutputType) {
        match out_type {
            ProcessOutputType::STDERR => {
                if self.stderr.len() > self.output_max_lines {
                    self.stderr.pop();
                }
                //println!("err: {}", data);
                self.stderr.push(data)
            },
            ProcessOutputType::STDOUT => {
                if self.stdout.len() > self.output_max_lines {
                    self.stdout.pop();
                }
                //println!("out: {}", data);
                self.stdout.push(data)
            }
        }
    }

    pub fn set_pid(&mut self, pid: u32) {
        self.pid = pid
    }
}

impl Default for ProcessState {
    fn default() -> Self {
        Self { stderr: Vec::new(), stdout: Vec::new(), output_max_lines: 32768, pid: 0 }
    }
}

enum ProcessOutputType {
    STDERR,
    STDOUT
}
