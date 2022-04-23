use std::{
    fs::File,
    io::{self, Read, Write},
};

pub enum ValueType {
    String,
    Number,
    Float,
    CustomStringFormat(String), // where the String is appended to the front of the found string
    Function,
    Table,
}

pub struct Options {
    pub old_name: String,
    pub new_name: String,
    pub value_type: ValueType,
    pub found: bool,
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

pub fn get_options() -> Vec<Options> {
    let options_vec = vec![
        Options::new("name", "Name", ValueType::String),
        Options::new("fullname", "FullName", ValueType::String),
        Options::new("category", "Category", ValueType::String),
        Options::new("description", "Description", ValueType::String),
        Options::new("quickdesc", "QuickDescription", ValueType::String),
        Options::new("cansprint", "CanSprint", ValueType::String),
        Options::new("type", "Type", ValueType::String),
        Options::new("cancrouch", "CanCrouch", ValueType::String),
        Options::new("cost", "WeaponCost", ValueType::Number),
        Options::new("slot", "Slot", ValueType::Number),
        Options::new("barrels", "NumBarrels", ValueType::Number),
        Options::new("handles", "NumHandles", ValueType::Number),
        Options::new("walkspeedreduce", "WalkspeedReduce", ValueType::Number),
        Options::new("batterymin", "BatteryDepletionMin", ValueType::Float),
        Options::new("batterymax", "BatteryDepletionMax", ValueType::Float),
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

pub fn initialize_file_header_types(f: &mut File) -> io::Result<()> {
    let mut top = File::open("top").expect("Top file should be able to open");
    let mut buf = String::new();
    top.read_to_string(&mut buf)?;

    f.write_all(buf.as_bytes())?;

    Ok(())
}
