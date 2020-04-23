use config::Config as Conf;
use config::ConfigError;
use thiserror::Error;
use secstr::SecUtf8;
use serde::Deserializer;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error("platform does not have a valid home directory")]
    Platform,
}

#[derive(Deserialize)]
pub struct Config {
    token: SecUt8,
}

static PREFIX: &'static str = "waah_";
static QUALIFIER: &'static str = "io";
static ORGANISATION: &'static str = "themadprofessor";
static APPLICATION: &'static str = "waah_bot";

impl Config {
    pub fn new() -> Result<Config, Error> {
        let dirs = directories::ProjectDirs::from(QUALIFIER, ORGANISATION, APPLICATION)
            .ok_or_else(|| Error::Platform)?;

        let mut conf = Conf::new();

        conf.merge(config::File::with_name(
            dirs.config_dir()
                .join(APPLICATION + ".toml")
                .to_str()
                .unwrap(),
        ))
        .map_err(Error::Config)?;
        conf.merge(config::Environment::with_prefix(PREFIX))
            .map_err(Error::Config)?;

        conf.try_into().map_err(Error::Config)
    }
}

fn deserialize_secstr<'de, D>(de: D) -> Result<SecUt8, D::Error> where D: Deserializer<'de> {
    String::deserialize(de)?.into()
}