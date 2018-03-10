use std::default::Default;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::thread;
use std::sync::mpsc;


#[derive(Debug)]
pub struct OpenVPN {
}

impl Default for OpenVPN {
    fn default() -> Self {
        Self { }
    }
}

#[derive(Debug)]
struct ProcessOutput {
    stderr: String,
    stdout: String,
    cap: usize
}

impl ProcessOutput {
    pub fn append_line(&mut self, data: &String, out_type: ProcessOutputType) {
        match out_type {
            ProcessOutputType::STDERR => self.stderr.push_str(data),
            ProcessOutputType::STDOUT => self.stdout.push_str(data)
        }
    }
}

impl Default for ProcessOutput {
    fn default() -> Self {
        Self { stderr: String::new(), stdout: String::new(), cap: 32768 }
    }
}

enum ProcessOutputType {
    STDERR,
    STDOUT
}

impl OpenVPN {
    pub fn run(&self) {
        self.start();
    }

    pub fn start(&self) {
        self.execute("./1.sh");
    }

    fn execute(&self, executable: &'static str) {
        let progname = executable.to_string();

        let _handle = thread::spawn(|| {
            let mut child = Command::new(progname)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap();

            let out = BufReader::new(child.stdout.take().unwrap());
            let err = BufReader::new(child.stderr.take().unwrap());

            let mut procout = ProcessOutput::default();

            let (tx, rx) = mpsc::channel();
            let _thread_err = thread::spawn(move || {
                err.lines().for_each(|line|
                    //println!("err: {}", line.unwrap())
                    tx.send(line.unwrap()).unwrap()
                );
            });
            for line in rx {
                procout.append_line(&line, ProcessOutputType::STDERR)
            }

            out.lines().for_each(|line|
                //println!("out: {}", line.unwrap())
                procout.append_line(&line.unwrap(), ProcessOutputType::STDOUT)
            );

            let status = child.wait().unwrap();
            println!("{}", status);
            println!("{:?}", procout);

        });
    }

}
