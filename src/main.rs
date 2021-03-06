use std::{
    fs::File,
    io::{self, Read, Write},
};

mod parsers;
use parsers::*;

mod initializers;
use initializers::*;

const BLACKLISTED_WEAPONS: &[&str] = &[
    "GPR",
    "PSTL",
    "BUGSPRAY",
    "NJ-CT",
    "Right Arm",
    "Right Shoulder",
    "Left Arm",
    "Left Shoulder",
    "firing",
    "damage",
    "sitting",
    "TEST",
];

fn handle_weapon_stats(key_name: String, reader: &mut Vec<String>) -> io::Result<f64> {
    let mut options = get_options();

    let file_name = key_name
        .strip_prefix("[\"")
        .unwrap()
        .strip_suffix("\"]")
        .expect(&format!("To be able to strip {}", key_name));

    let mut stats_file = File::create(format!("stats/{}.lua", file_name))
        .expect(&format!("failed to create file {}", key_name));

    initialize_file_header_types(&mut stats_file)?;
    stats_file.write_all(b"return {\n")?;

    let mut fire_rate = 0.0;

    while !reader.is_empty() {
        let str = reader
            .pop()
            .expect("should pop safely")
            .trim_end()
            .to_string();

        if str == "\t}," {
            break;
        }

        for option in &mut options {
            if !option.found {
                match &option.value_type {
                    ValueType::String => {
                        if let Some(result_str) = parse_string(&str, &option.old_name) {
                            stats_file.write(
                                format!(
                                    "{}{} = {},\n",
                                    "\t".repeat(START_TABS - 1),
                                    option.new_name,
                                    result_str
                                )
                                .as_bytes(),
                            )?;

                            option.found = true;
                            break;
                        }
                    }
                    ValueType::Number => {
                        if let Some(num) = parse_number(&str, &option.old_name) {
                            if option.new_name == "Slot" {
                                if num == 1 || num == 2 {
                                    stats_file.write_all(
                                        format!(
                                            "{}AmmoType = AmmoType.Battery,\n",
                                            "\t".repeat(START_TABS - 1)
                                        )
                                        .as_bytes(),
                                    )?;
                                }
                            }

                            stats_file.write(
                                format!(
                                    "{}{} = {},\n",
                                    "\t".repeat(START_TABS - 1),
                                    option.new_name,
                                    num
                                )
                                .as_bytes(),
                            )?;
                            option.found = true;
                            break;
                        }
                    }
                    ValueType::CustomStringFormat(format) => {
                        if let Some(mut result_str) = parse_string(&str, &option.old_name) {
                            result_str = result_str
                                .strip_prefix("\"")
                                .expect("should have a \"")
                                .to_string()
                                .strip_suffix("\"")
                                .expect("should have a \"")
                                .to_string();

                            stats_file.write(
                                format!(
                                    "{}{} = {},\n",
                                    "\t".repeat(START_TABS - 1),
                                    option.new_name,
                                    format!("{}{}", format, result_str)
                                )
                                .as_bytes(),
                            )?;

                            if option.new_name.eq("BulletType") {
                                stats_file.write(
                                    format!(
                                        "{}{} = {}\n",
                                        "\t".repeat(START_TABS - 1),
                                        "BulletCache",
                                        "Caches.DefaultCache,\n"
                                    )
                                    .as_bytes(),
                                )?;
                            }

                            option.found = true;
                            break;
                        }
                    }
                    ValueType::Function => {
                        let result = parse_function(
                            &str,
                            reader,
                            &option.old_name,
                            &option.new_name,
                            &mut stats_file,
                        )?;
                        if result {
                            option.found = true;
                            break;
                        }
                    }
                    ValueType::Table => {
                        let result = parse_table(
                            &str,
                            reader,
                            &option.old_name,
                            &option.new_name,
                            &mut stats_file,
                        )?;
                        if result {
                            option.found = true;
                            break;
                        }
                    }
                    ValueType::Float => {
                        if let Some(num) = parse_float(&str, &option.old_name) {
                            if option.new_name == "FireRate" {
                                fire_rate = num.into();
                            }

                            stats_file.write(
                                format!(
                                    "{}{} = {},\n",
                                    "\t".repeat(START_TABS - 1),
                                    option.new_name,
                                    num
                                )
                                .as_bytes(),
                            )?;
                            option.found = true;
                            break;
                        }
                    }
                }
            }
        }
    }

    stats_file.write_all(b"}\n")?;

    Ok(fire_rate)
}

fn handle_reader(buffer: &mut Vec<String>) {
    let mut highest_fire_rate = 0.0;
    while buffer.len() > 0 {
        let str = buffer.pop().expect("should pop safely");

        if str.find("local module") != None {
            continue;
        } else if str.starts_with("--") {
            continue;
        }

        if let Some(key) = parse_key(&str) {
            let mut should_continue = false;

            for string in BLACKLISTED_WEAPONS {
                if key.find(string) != None {
                    should_continue = true;
                    break;
                }
            }

            if should_continue {
                continue;
            }

            let res = handle_weapon_stats(key, buffer);

            match res {
                Ok(fire_rate) => {
                    if highest_fire_rate < fire_rate {
                        highest_fire_rate = fire_rate;
                    }
                }
                Err(_) => {}
            }
        }
    }

    println!("highest fire rate is {}", highest_fire_rate);
}

fn main() {
    let mut stats_file = File::open("stats.txt").expect("File should be able to open");
    let mut buf = String::new();
    stats_file
        .read_to_string(&mut buf)
        .expect("to be able to read from file");

    let mut buf_vec: Vec<String> = buf.split("\n").map(|v| v.to_string()).collect();
    buf_vec.reverse();

    handle_reader(&mut buf_vec);
}
