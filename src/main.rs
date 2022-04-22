use std::{
    fs::File,
    io::{self, Read, Write},
};

mod parsers;
use parsers::*;

enum ValueType {
    String,
    Number,
    CustomStringFormat(String), // where the String is appended to the front of the found string
    Function(u16),              // where the u16 is the expected amount of lines
    Table(u16),                 // where the u16 is the expect amount of lines until a }
}

struct Options {
    old_name: String,
    new_name: String,
    value_type: ValueType,
}

impl Options {
    pub fn new(old_name: &str, new_name: &str, val: ValueType) -> Options {
        Options {
            old_name: old_name.to_string(),
            new_name: new_name.to_string(),
            value_type: val,
        }
    }
}

fn get_options() -> Vec<Options> {
    let options_vec = vec![
        Options::new("name", "Name", ValueType::String),
        Options::new("fullname", "FullName", ValueType::String),
        Options::new("category", "Category", ValueType::String),
        Options::new("description", "Description", ValueType::String),
        Options::new("quickdesc", "QuickDescription", ValueType::String),
        Options::new("cost", "WeaponCost", ValueType::Number),
        Options::new("slot", "Slot", ValueType::Number),
        Options::new("barrels", "NumBarrels", ValueType::Number),
        Options::new("handles", "NumHandles", ValueType::Number),
        Options::new(
            "holster",
            "Holster",
            ValueType::CustomStringFormat("Holsters.".to_string()),
        ),
    ];

    options_vec
}

fn handle_weapon_stats(key_name: String, reader: &mut Vec<String>) -> io::Result<()> {
    let options = get_options();

    let file_name = key_name
        .strip_prefix("[\"")
        .unwrap()
        .strip_suffix("\"]")
        .unwrap();

    let mut stats_file = File::create(format!("stats/{}", file_name))
        .expect(&format!("failed to create file {}", key_name));

    stats_file.write_all(format!("{} = {{\n", key_name).as_bytes())?;

    while !reader.is_empty() {
        let str = reader.pop().expect("should pop safely");

        if str == "}" {
            break;
        }

        for option in options.iter() {
            match &option.value_type {
                ValueType::String => {
                    if let Some(result_str) = check_for_string(&str, &option.old_name) {
                        stats_file
                            .write(format!("\t{} = {}\n", option.new_name, result_str).as_bytes())?;
                        break;
                    }
                }
                ValueType::Number => {
                    if let Some(num) = check_for_number(&str, &option.old_name) {
                        stats_file.write(format!("\t{} = {}\n", option.new_name, num).as_bytes())?;
                        break;
                    }
                }
                ValueType::CustomStringFormat(format) => {
                    if let Some(mut result_str) = check_for_string(&str, &option.old_name) {
                        result_str = result_str
                            .strip_prefix("\"")
                            .expect("should have a \"")
                            .to_string()
                            .strip_suffix("\"")
                            .expect("should have a \"")
                            .to_string();

                        stats_file.write(
                            format!(
                                "\t{} = {}\n",
                                option.new_name,
                                format!("{}{}", format, result_str)
                            )
                            .as_bytes(),
                        )?;
                        break;
                    }
                }
                ValueType::Function(_) => todo!(),
                ValueType::Table(_) => todo!(),
            }
        }
    }

    stats_file.write_all(b"}\n")?;

    Ok(())
}

fn handle_reader(buffer: &mut Vec<String>) {
    while buffer.len() > 0 {
        let str = buffer.pop().expect("should pop safely");

        if str.find("local module") != None {
            continue;
        }

        if let Some(key) = check_for_key(&str) {
            let _res = handle_weapon_stats(key, buffer);
        }
    }
}

fn main() {
    let mut stats_file = File::open("stats_small.txt").expect("File should be able to open");
    let mut buf = String::new();
    stats_file
        .read_to_string(&mut buf)
        .expect("to be able to read from file");

    let mut buf_vec: Vec<String> = buf.split("\n").map(|v| v.to_string()).collect();
    buf_vec.reverse();

    handle_reader(&mut buf_vec);
}
