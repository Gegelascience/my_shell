use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::path::Path;
use std::env;
use std::process::Command;

fn main() {
    loop {
        print!("> ");
        stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // recuperation de la commande et des parametres avec trim
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;


        match command {
            "cd" => {
                // backup /
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            },
            "exit" => return,
            command => {
                let child = Command::new(command)
                    .args(args)
                    .spawn();


                // gestion d'une commande inconnue
                match child {
                    Ok(mut child) => { child.wait(); },
                    Err(e) => eprintln!("{}", e),
                };
            }
        }         

    }
    
}