use std::{fs::File, io::Read};

mod parsers;
use parsers::*;

fn handle_weapon_stats(key_name: String, reader: &mut Vec<String>) {
    let file_name = key_name
        .strip_prefix("[\"")
        .unwrap()
        .strip_suffix("\"]")
        .unwrap();
    let mut _stats_file = File::create(format!("stats/{}", file_name))
        .expect(&format!("failed to create file {}", key_name));

    while !reader.is_empty() {
        let str = reader.pop().expect("should pop safely");

        if str == "}" {
            break;
        }

        if let Some(name) = check_for_string(&str, "name") {
            println!("{}", name);
        } else if let Some(full_name) = check_for_string(&str, "fullname") {
            println!("{}", full_name);
        } else if let Some(category) = check_for_string(&str, "category") {
            println!("{}", category);
        } else if let Some(description) = check_for_string(&str, "description") {
            println!("{}", description);
        } else if let Some(quick_desc) = check_for_string(&str, "quickdesc") {
            println!("{}", quick_desc);
        } else if let Some(default_cost) = check_for_number(&str, "defaultcost") {
            println!("{}", default_cost);
        } else if let Some(cost) = check_for_number(&str, "cost") {
            println!("{}", cost);
        }
    }
}

fn handle_reader(buffer: &mut Vec<String>) {
    while buffer.len() > 0 {
        let str = buffer.pop().expect("should pop safely");

        if str.find("local module") != None {
            continue;
        }

        if let Some(key) = check_for_key(&str) {
            handle_weapon_stats(key, buffer);
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
