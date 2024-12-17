use std::fs;
use std::io::Write;

pub fn init_config_file() {
    let mut config_path = dirs::home_dir().expect("Failed to get home directory");
    config_path.push(".cuuri/config.toml");

    if !config_path.exists() {
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create directories for config file");
        }
        let mut file = fs::File::create(&config_path).expect("Failed to create config file");

        writeln!(file, "openai_api_key = \"\"").expect("Failed to write default API key");
        writeln!(file, "default_model = \"gpt-3.5-turbo\"").expect("Failed to write default model");

        println!("Configuration file created at {:?}", config_path);
    }

}
