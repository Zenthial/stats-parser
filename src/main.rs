use std::{
    fs::File,
    io::{Result, BufReader, BufRead},
};

use parsers::*;

mod parsers;

fn main() -> Result<()> {
    let stats_file = File::open("stats_small.txt")?;
    let reader = BufReader::new(stats_file);

    for line in reader.lines() {
        let str = line?;

        if str.find("local module") != None {
            continue;
        }

        if let Some(key) = check_for_key(&str) {
            println!("{}", key);
        } else if let Some(name) = check_for_string(&str, "name") {
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
 
    Ok(())
}
