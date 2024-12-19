use std::fs;
use std::io::{self, Write};

pub fn init_config_file() -> Result<(), io::Error> {
    let mut config_path = dirs::home_dir().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Failed to get home directory"))?;
    config_path.push(".cuuri/config.toml");

    if !config_path.exists() {
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let mut file = fs::File::create(&config_path)?;
        writeln!(file, "openai_api_key = \"\"")?;
        writeln!(file, "default_model = \"gpt-3.5-turbo\"")?;

        println!("Configuration file created at {:?}", config_path);
    }

    Ok(())
}