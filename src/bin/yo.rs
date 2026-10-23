use std::io;
use std::io::Write;

use yo::tokinizer;

const REPL: &'static str = ">";
const REPL_ERROR_PREFIX: &'static str = "yo :: [repl] :: ";

fn main() -> io::Result<()> {
    'repl: loop {
        print!("{} ", REPL);
        io::stdout()
            .flush()
            .expect(&format!("{}stdout flushing error", REPL_ERROR_PREFIX).to_string());

        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect(&format!("{}input reading error", REPL_ERROR_PREFIX).to_string());
        
        match input.as_str().trim() {
            "/quit" => break 'repl,
            s if s.starts_with("/file ") => {
                let arg = &s[6..];
                if let Ok(input) = std::fs::read_to_string(arg) {
                    let mut lex = tokinizer::Tokinizer::new(input);
                    let toks = lex.tokinize();
                    println!("{:?}", toks)
                } else {
                    println!("{}file reading error", REPL_ERROR_PREFIX)
                }
            },
            _ => {
                let mut lex = tokinizer::Tokinizer::new(input.clone());
                let toks = lex.tokinize();
                println!("{:?}", toks);
            }
        }
    }
    Ok(())
}