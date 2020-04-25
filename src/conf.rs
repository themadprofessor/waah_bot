use config::Config as Conf;
use config::ConfigError;
use log::LevelFilter;
use secstr::SecUtf8;
use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error("platform does not have a valid home directory")]
    Platform,
}

#[derive(Deserialize, PartialEq)]
pub struct Config {
    pub discord_token: SecUtf8,
    pub imgur_id: String,
    #[serde(default = "default_log_level")]
    pub log_level: LevelFilter,
}

const PREFIX: &str = "waah_";
const QUALIFIER: &str = "io";
const ORGANISATION: &str = "themadprofessor";
const APPLICATION: &str = "waah_bot";

impl Config {
    pub fn new() -> Result<Config, Error> {
        let dirs = directories::ProjectDirs::from(QUALIFIER, ORGANISATION, APPLICATION)
            .ok_or_else(|| Error::Platform)?;

        let mut conf = Conf::new();

        conf.merge(config::File::with_name(
            dirs.config_dir().join("waah_bot.toml").to_str().unwrap(),
        ))?;
        conf.merge(config::Environment::with_prefix(PREFIX))?;

        conf.try_into().map_err(Error::Config)
    }
}

fn default_log_level() -> LevelFilter {
    LevelFilter::Warn
}
