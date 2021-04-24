extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;
use colored::*;
use failure::ResultExt;
use exitfailure::ExitFailure;
use std::io::{self, Read, Write};

const MOOSE : &str =
"   \\|/    \\|/
     \\    /
      \\_/  ___   ___
      o o-'   '''   '
       O -.         |\
           | |'''| |
            ||   | |
            ||    ||
            /_\\  /_\\ ";

const COW : &str = "
           __n__n__
    .------`-\00/-'
   /  ##  ## (oo)
  / \\## __   ./
     |//YY \\|/
      |||   |||";

#[derive(StructOpt, Debug)]
struct Options {
    #[structopt(default_value = "Moose say Moo!")]
    /// Enter your msg here!
    message: String,

    #[structopt(short = "m", long = "moose")]
    /// Enter the animal you want to see!
    moose: bool,

    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// Use other ascii animals by placing in a .txt file
    /// by default animal.txt has monke art
    file: Option<std::path::PathBuf>,

    #[structopt(short = "i", long = "stdin")]
    /// Read msg from stdin
    stdin: bool,
}

fn main() -> Result<(), ExitFailure> {
    let options = Options::from_args();
    let mut message = "".to_string();
    if options.stdin {
        io::stdin().read_to_string(&mut message);
        message.pop();
    }
    else{
        message = options.message;
    }
    // let message = std::env::args().nth(1)
    //     .expect("Missing text to be said. Usage: animalsay <msg>");

    if options.moose {
        print_message_bubble(message);
        println!("{}", MOOSE.red());
    }
    else {
        match &options.file {
            Some(path) => {
                let alt = std::fs::read_to_string(path)
                .with_context(|_| format!("could not find file {:?}", path))?;
                print_message_bubble(message);
                println!("{}", alt.to_string().bright_yellow());
            }
            None => {
               print_message_bubble(message);
               println!("{}", COW.green());
            }
        }
    }
    Ok(())
}

fn print_message_bubble(message: String) {
    if message.to_lowercase().contains("fuck") {
        eprintln!("Animals don't curse!!!");
    }
    else{
        println!(" {} ", "-".repeat(message.chars().count() + 1));
        println!("| {} |", message);
        println!(" {} ", "-".repeat(message.chars().count() + 1));
        println!("   \\   |");
        println!("    \\  |");
        println!("     \\ |");
        println!("      \\|");
    }

}