use std::fs::File;
use std::io::Read;

use serde_yaml::from_str;

#[derive(Deserialize, Debug)]
pub struct Config;

impl Config {
    /// Load config using path specified in options
    pub fn load(opts: &::cli::Options) -> Result<Config, Box<::std::error::Error>> {
        // Read file to string
        let mut raw = String::new();
        let mut f = File::open(&opts.config_path)?;
        f.read_to_string(&mut raw)?;

        // Parse as yaml
        from_str(&raw).map_err(From::from)
    }
}
