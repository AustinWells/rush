extern crate rustyline;
extern crate regex;

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

                /* For testing purposes prints token stream to stdout */
                let line_chars = &mut line.chars();
                let mut tok = parse::get_token(line_chars);
                while (tok as i32) != (Token::EOL as i32) {
                    println!("{:?}", tok);
                    tok = parse::get_token(line_chars);
                }
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
