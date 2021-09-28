use std::env;
use rand::Rng;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::error::Error;
use std::io::ErrorKind;

use serde_json;

fn main() -> Result<(), Box<dyn Error>> {
    let mut just_created = true;

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(".map.json")
        .unwrap_or_else(|error| {
            if error.kind() == ErrorKind::AlreadyExists {
                just_created = false;
                OpenOptions::new()
                .read(true)
                .write(true)
                .append(false)
                .open(".map.json")
                .unwrap_or_else(|error| {
                    panic!("Problem with the file: {:?}", error);
                })
            } else {
                panic!("Problem with the file: {:?}", error);
            }
        });

    let mut pass_map: HashMap::<String, String> = match just_created {
        true => HashMap::new(),
        false => serde_json::from_reader(&file)?
    };

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please specify a command.");
    }

    match args[1].as_str() {
        "g" => {
            if args.len() < 3 {
                panic!("Please specify a password name.");
            }
    
            let name = &args[2];
    
            let mut psize: usize = rand::thread_rng().gen_range(8..15);
    
            if args.len() > 3 {
                let pass_len = &args[3];
                if let Ok(plen) = pass_len.parse::<usize>() {
                    psize = plen;
                } else {
                    panic!("Please make sure the length value is a valid unsigned integer.");
                }
            }
    
            let letters = &[
                'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g',
                'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n',
                'O', 'o', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u',
                'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y', 'Z', 'z'
            ];
    
            let mut pass_buf = String::new();
            for _ in 0..psize {
                pass_buf.push(letters[rand::thread_rng().gen_range(0..letters.len())]);
            }
    
            // pass_buf -> insert borrowed. no longer needs to be used.
            pass_map.insert(name.to_string(), pass_buf);
        }
        "o" => {
            if args.len() < 3 {
                panic!("Please specify a password name.");
            }

            let name = &args[2];

            if pass_map.len() == 0 {
                pass_map.insert(name.to_string(), "Password not found".to_string());
            }

            println!("Password for {}: {}", name, pass_map.get(name)
                .unwrap_or(&"Password not found".to_string()));
        }
        "a" => {
            if args.len() < 4 {
                panic!("Please specify a password name and matching password.");
            }

            let name = &args[2];
            let password = &args[3];

            pass_map.insert(name.to_string(), password.to_string());
        }
        "d" => {
            if args.len() < 3 {
                panic!("Please specify a password name.");
            }

            let name = &args[2];

            if pass_map.len() <= 1 {
                pass_map.insert(name.to_string(), "Password not found".to_string());
            }

            pass_map.remove(name);
        }
        _ => {
            panic!("Not a recognized command!");
        }
    };

    let c_file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(false)
        .truncate(true)
        .open(".map.json")?;
    serde_json::to_writer(&c_file, &pass_map)?;

    Ok(())
}
