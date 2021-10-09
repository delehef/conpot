//! The most simplest examples of how to use conpot

extern crate conpot;

#[macro_use]
extern crate serde_derive;

use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
struct ConpotConfig {
    name: String,
    comfy: bool,
    foo: i64,
}

impl Default for ConpotConfig {
    fn default() -> Self {
        ConpotConfig {
            name: "Unknown".to_string(),
            comfy: true,
            foo: 42,
        }
    }
}

fn main() -> Result<(), conpot::ConpotError> {
    let cfg: ConpotConfig = conpot::load("conpot_simple_app", None)?;
    let file = conpot::get_configuration_file_path("conpot_simple_app", None)?;
    println!("The configuration file path is: {:#?}", file);
    println!("The configuration is:");
    println!("{:#?}", cfg);
    println!("The wrote toml file content is:");
    let mut content = String::new();
    std::fs::File::open(&file)
        .expect("Failed to open toml configuration file.")
        .read_to_string(&mut content)
        .expect("Failed to read toml configuration file.");
    println!("{}", content);
    let cfg = ConpotConfig {
        name: "Test".to_string(),
        ..cfg
    };
    conpot::store("conpot_simple_app",None, &cfg)?;
    println!("The updated toml file content is:");
    let mut content = String::new();
    std::fs::File::open(&file)
        .expect("Failed to open toml configuration file.")
        .read_to_string(&mut content)
        .expect("Failed to read toml configuration file.");
    println!("{}", content);
    let _cfg = ConpotConfig {
        name: "Test".to_string(),
        ..cfg
    };
    std::fs::remove_dir_all(file.parent().unwrap())
        .expect("Failed to remove directory");
    Ok(())
}
