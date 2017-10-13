extern crate rustyline;

pub mod parse;
use parse::*;

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let mut rl = Editor::<()>::new();

    if let Err(_) = rl.load_history("/home/wolfe/.rsh.hist") {
        println!("No previous history");
    }
    loop {
        let readline = rl.readline("$ ");

        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);

                parse::parsed_line(line);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CC");
            }
            Err(ReadlineError::Eof) => {
                println!("CD");
                break
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history("/home/wolfe/.rsh.hist").unwrap();
}
