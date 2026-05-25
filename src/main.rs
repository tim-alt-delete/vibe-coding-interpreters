use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor};

use std::env;
use std::fs;
use std::io;

// declares module.
// It tells the compiler to load that module from a file 
// (either src/lexer.rs or src/lexer/mod.rs).
mod lexer;
use crate::lexer::{Lexer, TokenKind};

mod parser;
use crate::parser::{Parser, Stmt, Expr};

// mod interpreter;

fn run(source: &str) {
    //println!("{}", source);

    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();

    // Lexing loop with error handling
    // while let Ok(token) = lexer.next_token() {
    //     tokens.push(token.clone());
    //     println!("{:?}", token);

    //     if matches!(token.kind, TokenKind::Eof) {
    //         break;
    //     }
    // }
    loop {
        match lexer.next_token() {
            Ok(token) => {
                println!("{:?}", token); // Optional: print tokens as we lex
                tokens.push(token.clone());
                if matches!(token.kind, TokenKind::Eof) { break; }
            }
            Err(err) => {
                eprintln!("Lex error: {}", err);
                std::process::exit(1); // or continue to skip/attempt recovery
            }
        }
    }

    //Parsing tokens into AST
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(ast) => {
            println!("{:#?}", ast); // Optional: print AST
            for stmt in ast {
                match stmt {
                    Stmt::Expression(expr) => {
                        //let value = interpreter::eval(&expr);
                        // println!("=> {}", value); // Display result
                    }
                    _ => {
                        println!("(unhandled stmt)");
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Parse error: {:?}", e);
        }
    }
}

fn run_file(filename: &str) -> io::Result<()> {
    let source = fs::read_to_string(filename)?;
    run(&source);
    Ok(())
}

fn repl() -> rustyline::Result<()> {
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
    // $ cargo run foo bar
    // ["target/debug/my_program", "foo", "bar"]
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        if let Err(e) = run_file(&args[1]) {
            eprintln!("Error in run_file: {}", e);
            std::process::exit(1)
        }
    } else if args.len() == 1 {
        if let Err(e) = repl() {
            eprintln!("Error in REPL: {}", e);
            std::process::exit(1)
        }    
    } else {
        eprintln!("Usage: rox [file]");
        std::process::exit(1)
    }
}    