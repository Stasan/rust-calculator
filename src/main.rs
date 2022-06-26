use std::{env, process};
use calculator::PostfixNotation;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let postfix_notation_expression = PostfixNotation::from_infix_string(&config.infix_expression_string);
    let result = postfix_notation_expression.calculate();

    println!("You provided following expression: {}", config.infix_expression_string);
    println!("{}", postfix_notation_expression);
    println!("Result: {}", result);
}

pub struct Config {
    pub infix_expression_string: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 2 {
            return Err("Provide valid math expression using numbers and operator +-/* without space characters");
        }

        let infix_expression_string = args[1].clone();

        Ok(Config {
            infix_expression_string,
        })
    }
}
