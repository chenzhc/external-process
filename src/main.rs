#![allow(dead_code)]
use std::{io::{Read, Write}, process::Command};

fn main() {
    test_stdin();

}

fn test_stdin() {
    let mut head_cmd = Command::new("head");
    head_cmd.arg("-n 1");
    head_cmd.stdin(std::process::Stdio::piped());
    head_cmd.stdout(std::process::Stdio::piped());


    let input_data = "inputone\ninputwo".as_bytes();

    let mut  proc_handle = head_cmd.spawn().unwrap();
    let mut  stdin_handle = proc_handle.stdin.take().unwrap();

    let _result = stdin_handle.write_all(input_data);
     _ = proc_handle.wait();

    let mut  output_buffer = String::new();

    _ = proc_handle.stdout.unwrap().read_to_string(&mut output_buffer);
    
    println!("Result was: {}", output_buffer);


}


fn test_procs() {
    let mut p1 =  Command::new("which");
    p1.arg("python3");

    let proc_result = p1.output();

    if proc_result.is_ok() {
        let result = proc_result.ok().unwrap();
        println!("Was execution sucessfull? {}", result.status.success());
        if !result.status.success() {
            println!("Error occurred: {}", result.status.code().unwrap());
        } else {
            // std::io::stdout().write_all(&result.stdout).unwrap();
            println!("{}", String::from_utf8(result.stdout).unwrap());

        }
    }

    let mut p2 = Command::new("echo");
    p2.args(["$PATH"]);
    p2.stdout(std::process::Stdio::null());
    let mut p2_handle = p2.spawn().unwrap();

    println!("Doing some more work...");
    let proc_result = p2_handle.wait().unwrap();
    println!("Existed with code: {}", proc_result.code().unwrap());
    // println!("{}", String::from_utf8(proc_result));
}