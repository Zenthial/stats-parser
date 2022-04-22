use lazy_static::lazy_static;
use regex::Regex;

pub fn check_for_key(line: &String) -> Option<String> {
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

pub fn check_for_string(line: &String, search_string: &str) -> Option<String> {
    let new_line = line.trim();

    if new_line.starts_with(search_string) {
        let split: Vec<String> = new_line.split("=").map(|v| v.to_string()).collect();

        assert!(split.len() == 2);

        let name_untrimmed = split.get(1).expect("name to be gotten");
        let value = name_untrimmed
            .strip_suffix(",")
            .expect("this to work")
            .trim();

        return Some(value.to_string());
    }

    None
}

pub fn check_for_number(line: &String, search_string: &str) -> Option<i32> {
    let new_line = line.trim();

    if new_line.starts_with(search_string) {
        let split: Vec<String> = new_line.split("=").map(|v| v.to_string()).collect();

        assert!(split.len() == 2);

        let name_untrimmed = split.get(1).expect("name to be gotten");
        let str_value = name_untrimmed
            .strip_suffix(",")
            .expect("this to work")
            .trim();

        let int_val_result = str_value.parse::<i32>();
        match int_val_result {
            Ok(val) => return Some(val),
            Err(err) => {
                println!("{:?}", err);
                return None;
            }
        }
    }

    None
}
