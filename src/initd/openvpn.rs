use std::default::Default;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::thread;


#[derive(Debug)]
pub struct OpenVPN {
}

impl Default for OpenVPN {
    fn default() -> Self {
        Self { }
    }
}

#[derive(Debug)]
struct StringWithCap {
    storage: String,
    cap: usize
}

impl StringWithCap {
    pub fn append(&mut self, data: &String) {
        self.storage.push_str(data)
    }
}

impl Default for StringWithCap {
    fn default() -> Self {
        Self { storage: String::new(), cap: 1048576 }
    }
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

            let mut stdout = StringWithCap::default();

            let _thread_err = thread::spawn(move || {
                let mut stderr = StringWithCap::default();
                err.lines().for_each(|line|
                    //println!("err: {}", line.unwrap())
                    stderr.append(&line.unwrap())
                );
                println!("{:?}", stderr);
            });

            out.lines().for_each(|line|
                //println!("out: {}", line.unwrap())
                stdout.append(&line.unwrap())
            );

            let status = child.wait().unwrap();
            println!("{}", status);
            println!("{:?}", stdout);

        });
    }

}
