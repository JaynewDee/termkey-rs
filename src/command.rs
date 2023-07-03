use crate::errors::{ControlError, WriteError};
use std::fs::File;
use std::io::prelude::*;
use std::io::LineWriter;
use std::io::{stdin, stdout};
use std::process::exit;

//
//
//

pub fn get_cl_args() -> Vec<String> {
    std::env::args().collect()
}

fn process_args(args: &[String]) -> Result<(String, String), Box<dyn std::error::Error>> {
    if args.len() < 2 {
        println!("A minimum of (1) argument must be passed.");
        exit(0)
    }
    let command_input = args[1].trim().to_lowercase();

    let operation = match command_input.as_str() {
        "gen" | "g" | "keygen" => String::from("keygen"),
        "e" | "encrypt" => String::from("encrypt"),
        "d" | "decrypt" => String::from("decrypt"),
        _ => unimplemented!(),
    };

    let op_param = match operation.as_str() {
        "keygen" => None,
        "encrypt" | "decrypt" => Some(args[2].clone()),
        _ => unimplemented!(),
    };

    if let Some(param) = op_param {
        Ok((operation, param))
    } else {
        Ok((operation, String::new()))
    }
}

pub fn init_write_file(name: &str) -> Result<LineWriter<File>, WriteError> {
    let line_writer = LineWriter::new(create_file(name)?);
    Ok(line_writer)
}

fn create_file(name: &str) -> Result<File, WriteError> {
    let file = File::create(name)?;
    Ok(file)
}

pub fn control_flow(cl_args: Vec<String>) -> Result<String, ControlError> {
    let (op, param) = process_args(&cl_args).expect("Should have processed arguments.");
    //
    // let mut line_writer = init_write_file("cipher_key.bin").unwrap();

    println!("{}", &op);
    println!("{}", &param);

    Ok("EXIT".to_string())
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();

    stdin()
        .read_line(&mut buffer)
        .expect("Stdin should have read user input into string buffer.");

    flush_out();
    Some(buffer)
}

fn flush_out() {
    stdout()
        .flush()
        .expect("Should have flushed stdout stream ... ");
}

fn prompt_for(opt: &str) {
    match opt {
        "filename" => {
            println!("Enter a name for your output file:");
            flush_out();
        }
        _ => {
            println!("Reached wildcard match arm");
            flush_out();
        }
    }
}
