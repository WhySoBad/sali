use std::path::Path;
use dirs::home_dir;
use clap::Parser;

const CONFIG_PATH: &str = ".config/sali/config.yaml";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(long, short, default_value_t = get_default_config_path())]
    /// Alternative path to a config file
    pub config: String,

    #[arg(long, short)]
    /// Start the gtk inspector on application launch
    pub inspect: bool
}

fn get_default_config_path() -> String {
    let home_dir = home_dir().unwrap_or_default();
    let path = home_dir.join(Path::new(CONFIG_PATH));
    String::from(path.to_str().unwrap_or_default())
}