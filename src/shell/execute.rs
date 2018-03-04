use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::thread;


pub fn execute(command: String) {
    let handle = thread::spawn(|| {
        let mut child = Command::new(command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();

        let out = BufReader::new(child.stdout.take().unwrap());
        let err = BufReader::new(child.stderr.take().unwrap());

        let thread_err = thread::spawn(move || {
            err.lines().for_each(|line|
                println!("err: {}", line.unwrap())
            );
        });

        out.lines().for_each(|line|
            println!("out: {}", line.unwrap())
        );

        let status = child.wait().unwrap();
        println!("{}", status);
    });
}
