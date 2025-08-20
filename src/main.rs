use std::{io::Write, process::Command};

fn main() {
    
    let mut p1 =  Command::new("which");
    p1.arg("python3");

    let proc_result = p1.output();

    if proc_result.is_ok() {
        let result = proc_result.ok().unwrap();
        println!("Was execution sucessfull? {}", result.status.success());
        if !result.status.success() {
            println!("Error occurred: {}", result.status.code().unwrap());
        } else {
            std::io::stdout().write_all(&result.stdout).unwrap();
        }
    }

}
