use crate::{
    evaluator::{environment::Environment, eval},
    lexer::Lexer,
    parser::Parser,
};
use std::{
    io::{self, Write},
    rc::Rc,
};

#[derive(Debug)]
enum MetaCommand {
    Help,
    Clear,
    Ping,
    Exit,
}

pub struct Repl {
    running: bool,
}

impl Repl {
    pub fn new() -> Self {
        Repl { running: true }
    }
    pub fn start(&mut self) -> io::Result<()> {
        Repl::print_information();

        Repl::print_prompt();
        let mut input = Repl::read_input()?;
        let mut env = Environment::new();

        while self.running {
            if Repl::input_is_meta_command(&input) {
                let cmd = Repl::parse_command(&input);

                if let Some(cmd) = cmd {
                    match cmd {
                        MetaCommand::Exit => {
                            self.running = false;
                            continue;
                        }
                        _ => Repl::do_meta_command(&cmd),
                    }
                }
            } else {
                let lexer = Lexer::new(&input);
                let mut parser = Parser::new(lexer);
                let prog = parser.parse_program();

                if parser.errors.len() > 0 {
                    for e in parser.errors {
                        println!("{}", e);
                    }
                } else {
                    match eval(&prog, Rc::clone(&env)) {
                        Some(v) => {
                            println!("{:?}", v);
                        }
                        None => {
                            println!();
                        }
                    }
                }
            }

            Repl::print_prompt();
            input = Repl::read_input()?;
        }

        Ok(())
    }

    fn parse_command(input: &str) -> Option<MetaCommand> {
        match input {
            ".help" => Some(MetaCommand::Help),
            ".clear" => Some(MetaCommand::Clear),
            ".ping" => Some(MetaCommand::Ping),
            ".exit" => Some(MetaCommand::Exit),
            _ => None,
        }
    }

    fn do_meta_command(cmd: &MetaCommand) {
        match cmd {
            MetaCommand::Help => {
                Repl::print_help();
            }
            MetaCommand::Clear => {
                Repl::clear_repl();
            }
            MetaCommand::Ping => {
                println!("pong");
            }
            MetaCommand::Exit => {}
        }
    }

    fn input_is_meta_command(input: &str) -> bool {
        std::str::from_utf8(input.as_bytes().get(0..1).unwrap()).unwrap() == "."
    }

    fn read_input() -> io::Result<String> {
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }

    fn print_prompt() {
        print!("> ");
        io::stdout().flush();
    }

    fn clear_repl() {
        const CLEAR: &str = "\x1B[2J\x1B[0;0H";
        print!("{CLEAR}");
    }

    fn print_information() {
        println!("Welcome to _");
        println!("Type \".help\" for more information.");
        io::stdout().flush();
    }

    fn print_help() {
        let help = vec![
            (".clear", "Clear the REPL"),
            (".exit", "Exit the REPL"),
            (".help", "Print this help message"),
            (".ping", "Print \"pong\""),
        ];

        const PADDING: usize = 3;
        let max_cmd_len = help
            .iter()
            .map(|(cmd, _)| cmd.len())
            .max()
            .unwrap_or_default();
        let padding_size = PADDING + max_cmd_len;

        for (cmd, msg) in help {
            println!("{:<width$}{}", cmd, msg, width = padding_size);
        }

        println!();
    }
}
