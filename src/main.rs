use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use std::env;
use std::fs;
use std::io;

fn run(source: &str) {
    println!("{}", source);
    // interpreter logic goes here
    // let tokens = lex(&line);
    // dbg!(&tokens);
    // let ast = parse(tokens);
    // let result = eval(ast);
}


fn run_file(filename: &str) -> io::Result<()> {
    let source = fs::read_to_string(filename)?;
    println!("{}", source);
    run(&source);
    Ok(())
}


fn repl() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                run(&line);
                if line == "exit" {
                    break
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    #[cfg(feature = "with-file-history")]
    if let Err(e) = rl.save_history("history.txt") {
        eprintln!("Could not save history: {}", e);
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        if let Err(e) = run_file(&args[1]) {
            eprintln!("Error in run_file: {}", e);
        }
    } else if args.len() == 1 {
        if let Err(e) = repl() {
            eprintln!("Error in REPL: {}", e);
        }    
    } else {
        eprintln!("Usage: rox [file]");
    }
}    