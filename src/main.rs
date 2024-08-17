use std::{fs::File, io::Read};

use serde_yaml::*;
use arrata_lib::Quirk;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <name> <file_name>", args[0]);
        return;
    }

    read_write_to_file(args[1].clone(), args[2].clone());
}

fn read_write_to_file(name: String, file_name: String) {
    let mut f = match File::open(name) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to open file: {}", err);
            return;
        }
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => {},
        Err(err) => {
            eprintln!("Failed to read file: {}", err);
            return;
        }
    }

    let val = match serde_yaml::from_str::<Value>(&s) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Failed to parse YAML: {}", err);
            return;
        }
    };

    let quirks_sequence = match val.as_mapping() {
        Some(mapping) => match mapping.get("quirks") {
            Some(quirks) => match quirks.as_sequence() {
                Some(sequence) => sequence,
                None => {
                    eprintln!("Failed to get quirks sequence");
                    return;
                }
            },
            None => {
                eprintln!("Failed to get quirks");
                return;
            }
        },
        None => {
            eprintln!("Failed to get mapping");
            return;
        }
    };

    let quirks = quirks_sequence.iter().filter_map(|q| {
        match serde_yaml::from_value::<Quirk>(q.clone()) {
            Ok(quirk) => Some(quirk),
            Err(err) => {
                eprintln!("Failed to deserialize quirk: {}", err);
                None // Skip the quirk by returning None
            }
        }
    }).collect::<Vec<Quirk>>();

    let mut f = match File::create(file_name) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to create file: {}", err);
            return;
        }
    };
    match bincode::serialize_into(&mut f, &quirks) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("Failed to serialize quirks: {}", err);
        }
    }
}
