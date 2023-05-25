use std::io::{Write, Read};
use std::{env, io};
use std::fs::File;
use std::process::ExitCode;
use std::path::Path;
fn main() -> Result<(), ExitCode> {
    let args:Vec<String> = env::args().collect();
    match args.len() {
        2 => { run_file(Path::new(&args[1])).expect("Couldn't run file"); },
        1 => { run_prompt().expect("Couldn't run prompt command"); },
        _ => {
            eprintln!("Usage: rlox [script]");
            return Err(ExitCode::from(64));
        }
    }
    Ok(())
}

fn run_file<P: AsRef<Path>>(path: P) -> io::Result<()> {
    println!("Running file");
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    run(content);
    Ok(())
}

fn run_prompt() -> io::Result<()> {
    println!("Running prompt");
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().expect("Couldnt flush stdout");
        let mut line = String::new();
        match stdin.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => run(line),
            Err(e) => return Err(e),
        }
        
    }
    Ok(())
}


use rlux::scanner::Scanner;
use rlux::parser::Parser;
use rlux::interpreter::Interpreter;

fn run(source: String) {
    let mut scanner = Scanner::new(source);
    let mut parser = Parser::new(scanner.tokens().to_vec());
    let tree = parser.build_tree();
    // println!("{tree}");
    let interp = Interpreter::new();
    interp.interpret(tree);
    // for token in scanner.tokens() {
    //     println!("{token}");
    // }
}
