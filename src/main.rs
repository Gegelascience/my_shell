use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::path::Path;
use std::env;
use std::process::Command;
use std::process::Stdio;
use std::process::Child;

use winput::{Vk, Action};
use winput::message_loop;

use std::thread;
use std::sync::mpsc::channel;

fn main() {
    let mut last_input ="".to_string();
    let keyboard_receiver = message_loop::start().unwrap();
    let (tx,rx) = channel();

    thread::spawn(move || {
        loop {
           
            match keyboard_receiver.next_event() {
                message_loop::Event::Keyboard {
                    vk,
                    action: Action::Press,
                    ..
                } => {
                    if vk ==  Vk::UpArrow {
                        //print!("{:?}",last_input);
                        //input = last_input;
                        let old_input: String =rx.recv().unwrap();
                        
                        print!("{:?}",old_input.trim());
                        let _ = stdout().flush();
                    }
                    /*if vk == Vk::Escape {
                        println!("escape");
                    } else {
                        println!("{:?} was pressed!", vk);
                    }*/
                },
                _ => (),
            }
            
        }
    });
    loop {
        print!("> ");

        let _ = stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // recuperation de la commande et des parametres avec trim
        tx.send(input.clone()).unwrap();

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next()  {
            let mut parts = command.trim().split_whitespace();
            let command_opt = parts.next();

            // verification si commande
            match command_opt {
                Some(command) => {
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
                            let stdin = previous_command
                                .map_or(
                                    Stdio::inherit(),
                                    |output: Child| Stdio::from(output.stdout.unwrap())
                                );
        
                            let stdout = if commands.peek().is_some() {
                                // il y a une commande supplementaire il traiter
                                Stdio::piped()
                            } else {
                                // traitement derniere commande
                                Stdio::inherit()
                            };
        
                            let output = Command::new(command)
                                .args(args)
                                .stdin(stdin)
                                .stdout(stdout)
                                .spawn();
        
                            match output {
                                Ok(output) => { previous_command = Some(output); },
                                Err(e) => {
                                    previous_command = None;
                                    eprintln!("{}", e);
                                },
                            };
                        }
                    }
                    
                },
                None => break,
            }
            

           
        }

        if let Some(mut final_command) = previous_command {
            // attente resultat derniere commande
            let _ = final_command.wait();
        }

                 

    }
    
}
