use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Default)]
pub struct Blah {}

fn main() -> std::io::Result<()> {
    let mut args = env::args_os();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
    } else if args.len() == 2 {
        let s = args.nth(1).unwrap();
        println!("{:?}", s);
        println!("Running file...");
        run_file(s)?;
    } else {
        println!("Running prompt...");
        run_prompt()?;
    }
    println!("Exiting...");
    Ok(())
}

fn run_file(path: OsString) -> std::io::Result<()> {
    let mut file = File::open(path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    run(s);
    Ok(())
}

fn run_prompt() -> std::io::Result<()> {
    let quit = "quit\n".to_string();
    loop {
        let mut input = String::new();
        print!("> ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut input)?;

        if input.eq(&quit) { 
            break 
        }

        print!("Input was: {}", input);
    }
    Ok(())
}

fn run(_source: String) {}
