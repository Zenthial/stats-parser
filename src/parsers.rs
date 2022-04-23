use std::{
    fs::File,
    io::{Result, Write},
};

use lazy_static::lazy_static;
use regex::Regex;

pub const START_TABS: usize = 2;

pub fn parse_key(line: &String) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\[[^\]]*\]").unwrap();
    }

    let new_line = line.trim();
    let match_option = RE.find(new_line);

    if let Some(mtch) = match_option {
        let weapon_name = &new_line[mtch.range()];

        return Some(weapon_name.to_string());
    }

    None
}

pub fn parse_string(line: &String, search_string: &str) -> Option<String> {
    let new_line = line.trim();

    if new_line.starts_with(search_string) {
        let split: Vec<String> = new_line.split("=").map(|v| v.to_string()).collect();

        assert!(split.len() == 2);

        let name_untrimmed = split.get(1).expect("name to be gotten");
        let str_value;

        match name_untrimmed.strip_suffix(",") {
            Some(str) => str_value = str.trim(),
            None => str_value = name_untrimmed.trim(),
        }

        return Some(str_value.to_string());
    }

    None
}

pub fn parse_number(line: &String, search_string: &str) -> Option<i32> {
    let new_line = line.trim();

    if new_line.starts_with(search_string) {
        let split: Vec<String> = new_line.split("=").map(|v| v.to_string()).collect();

        assert!(split.len() == 2);

        let name_untrimmed = split.get(1).expect("name to be gotten");
        let str_value;

        match name_untrimmed.strip_suffix(",") {
            Some(str) => str_value = str.trim(),
            None => str_value = name_untrimmed.trim(),
        }

        let int_val_result = str_value.parse::<i32>();
        match int_val_result {
            Ok(val) => return Some(val),
            Err(err) => {
                println!("{} {:?}", str_value, err.to_string());
                return None;
            }
        }
    }

    None
}

pub fn parse_float(line: &String, search_string: &str) -> Option<f32> {
    let new_line = line.trim();

    if new_line.starts_with(search_string) {
        let split: Vec<String> = new_line.split("=").map(|v| v.to_string()).collect();

        assert!(split.len() == 2);

        let name_untrimmed = split.get(1).expect("name to be gotten");
        let str_value = name_untrimmed
            .strip_suffix(",")
            .expect("this to work")
            .trim();

        let int_val_result = str_value.parse::<f32>();
        match int_val_result {
            Ok(val) => return Some(val),
            Err(err) => {
                println!("{} {:?}", str_value, err.to_string());
                return None;
            }
        }
    }

    None
}

pub fn parse_function(
    line: &String,
    reader: &mut Vec<String>,
    func_name: &str,
    replace_name: &str,
    out_file: &mut File,
) -> Result<bool> {
    let new_line = line.trim();

    if new_line.starts_with(func_name) {
        let mut num_ends = 1;
        let mut tabs = START_TABS;

        let line_split: Vec<String> = new_line.split("=").map(|v| v.to_string()).collect();
        out_file.write_all(
            format!(
                "{}{} ={}\n",
                "\t".repeat(tabs - 1),
                replace_name,
                line_split.get(1).expect("second val should exist")
            )
            .as_bytes(),
        )?;

        while num_ends != 0 {
            let mut line = reader.pop().expect("line should pop cleanly");
            line = line.trim().to_string();

            if line.find("if") != None || line.find("while") != None {
                num_ends += 1;
                out_file.write_all(format!("{}{}\n", "\t".repeat(tabs), line).as_bytes())?;
                tabs += 1;
            } else if line.find("end") != None {
                num_ends -= 1;
                tabs -= 1;
                out_file.write_all(format!("{}{}\n", "\t".repeat(tabs), line).as_bytes())?;
            } else if line.find("else") != None {
                tabs -= 1;
                out_file.write_all(format!("{}{}\n", "\t".repeat(tabs), line).as_bytes())?;
                tabs += 1;
            } else {
                out_file.write_all(format!("{}{}\n", "\t".repeat(tabs), line).as_bytes())?;
            }
        }

        return Ok(true);
    }

    Ok(false)
}

pub fn parse_table(
    line: &String,
    reader: &mut Vec<String>,
    table_name: &str,
    replace_name: &str,
    out_file: &mut File,
) -> Result<bool> {
    let new_line = line.trim();

    if new_line.starts_with(table_name) {
        let mut num_ends = 1;
        let mut tabs = START_TABS;
        let line_split: Vec<String> = new_line.split("=").map(|v| v.to_string()).collect();
        out_file.write_all(
            format!(
                "{}{} ={}\n",
                "\t".repeat(tabs - 1),
                replace_name,
                line_split.get(1).expect("second val should exist")
            )
            .as_bytes(),
        )?;

        while num_ends != 0 {
            let mut line = reader.pop().expect("line should pop cleanly");
            line = line.trim().to_string();

            if line.find("{") != None {
                num_ends += 1;
                out_file.write_all(format!("{}{}\n", "\t".repeat(tabs), line).as_bytes())?;
                tabs += 1;
            } else if line.find("}") != None {
                num_ends -= 1;
                tabs -= 1;
                out_file.write_all(format!("{}{}\n", "\t".repeat(tabs), line).as_bytes())?;
            } else {
                out_file.write_all(format!("{}{}\n", "\t".repeat(tabs), line).as_bytes())?;
            }
        }

        return Ok(true);
    }

    Ok(false)
}
