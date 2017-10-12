extern crate rustyline;
extern crate regex;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use regex::Regex;

fn main() {
    let mut rl = Editor::<()>::new();
    let regex = Regex::new(r#"(?:[^\s,""']|[""'](?:\\.|[^""])*[""'])+"#).unwrap();

    if let Err(_) = rl.load_history("/home/wolfe/.rsh.hist") {
        println!("No previous history");
    }
    loop {
        let readline = rl.readline("$ ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                let match_iter = regex.find_iter(line.as_str());
                let l: Vec<&str> = match_iter.map(|v| v.as_str()).collect();
                println!("{:?}", l);
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
