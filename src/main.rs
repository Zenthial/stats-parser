use std::{
    fs::File,
    io::{self, Read, Write},
};

mod parsers;
use parsers::*;

enum ValueType {
    String,
    Number,
    Float,
    CustomStringFormat(String), // where the String is appended to the front of the found string
    Function,
    Table,
}

struct Options {
    old_name: String,
    new_name: String,
    value_type: ValueType,
    found: bool,
}

impl Options {
    pub fn new(old_name: &str, new_name: &str, val: ValueType) -> Options {
        Options {
            old_name: old_name.to_string(),
            new_name: new_name.to_string(),
            value_type: val,
            found: false,
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
        Options::new("cansprint", "CanSprint", ValueType::String),
        Options::new("cancrouch", "CanCrouch", ValueType::String),
        Options::new("cost", "WeaponCost", ValueType::Number),
        Options::new("slot", "Slot", ValueType::Number),
        Options::new("barrels", "NumBarrels", ValueType::Number),
        Options::new("handles", "NumHandles", ValueType::Number),
        Options::new("walkspeedreduce", "WalkspeedReduce", ValueType::Number),
        Options::new("batterymin", "BatteryDepletionMin", ValueType::Number),
        Options::new("batterymax", "BatteryDepletionMax", ValueType::Number),
        Options::new("shotsdeplete", "ShotsDeplete", ValueType::Number),
        Options::new(
            "holster",
            "Holster",
            ValueType::CustomStringFormat("Holsters.".to_string()),
        ),
        Options::new(
            "triggermode",
            "GunType",
            ValueType::CustomStringFormat("GunTypes.".to_string()),
        ),
        Options::new(
            "bullettype",
            "BulletType",
            ValueType::CustomStringFormat("BulletType.".to_string()),
        ),
        Options::new(
            "firemode",
            "FireMode",
            ValueType::CustomStringFormat("FireMode.".to_string()),
        ),
        Options::new("equipwait", "EquipTime", ValueType::Float),
        Options::new("chargewait", "ChargeWait", ValueType::Float),
        Options::new("firerate", "FireRate", ValueType::Float),
        Options::new("maxspread", "MaxSpread", ValueType::Float),
        Options::new("minspread", "MinSpread", ValueType::Float),
        Options::new("heatrate", "HeatRate", ValueType::Float),
        Options::new("cooltime", "CoolTime", ValueType::Float),
        Options::new("coolwait", "CoolWait", ValueType::Float),
        Options::new("headshotmultiplier", "HeadshotMultiplier", ValueType::Float),
        Options::new("vehiclemultiplier", "VehicleMultiplier", ValueType::Float),
        Options::new("calcDamage", "CalculateDamage", ValueType::Function),
        Options::new("handlewelds", "HandleWelds", ValueType::Table),
    ];

    options_vec
}

fn handle_weapon_stats(key_name: String, reader: &mut Vec<String>) -> io::Result<()> {
    let mut options = get_options();

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

        for option in &mut options {
            if !option.found {
                match &option.value_type {
                    ValueType::String => {
                        if let Some(result_str) = parse_string(&str, &option.old_name) {
                            stats_file.write(
                                format!("\t{} = {}\n", option.new_name, result_str).as_bytes(),
                            )?;
                            option.found = true;
                            break;
                        }
                    }
                    ValueType::Number => {
                        if let Some(num) = parse_number(&str, &option.old_name) {
                            stats_file
                                .write(format!("\t{} = {}\n", option.new_name, num).as_bytes())?;
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
                                    "\t{} = {}\n",
                                    option.new_name,
                                    format!("{}{}", format, result_str)
                                )
                                .as_bytes(),
                            )?;
                            option.found = true;
                            break;
                        }
                    }
                    ValueType::Function => {
                        parse_function(
                            &str,
                            reader,
                            &option.old_name,
                            &option.new_name,
                            &mut stats_file,
                        )?;
                    }
                    ValueType::Table => {
                        parse_table(
                            &str,
                            reader,
                            &option.old_name,
                            &option.new_name,
                            &mut stats_file,
                        )?;
                    }
                    ValueType::Float => {
                        if let Some(num) = parse_float(&str, &option.old_name) {
                            stats_file
                                .write(format!("\t{} = {}\n", option.new_name, num).as_bytes())?;
                            option.found = true;
                            break;
                        }
                    }
                }
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

        if let Some(key) = parse_key(&str) {
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
