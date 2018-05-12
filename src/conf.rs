use failure::Error;

pub struct Config {
    pub discord_token: String
}

impl Config {
    pub fn from_conf(conf: ::config::Config) -> Result<Config, Error> {
        Ok(Config {
            discord_token: conf.get_str("discord_token").map_err(|_| ::failure::err_msg("No discord token found"))?
        })
    }
}