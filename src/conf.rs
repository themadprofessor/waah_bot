use failure::Error;

pub struct Config {
    pub discord_token: String,
    pub name: String,
    pub cmd_char: String,
    //map: HashMap<&'a str, Vec<&'a str>>
}

impl Config {
    pub fn from_conf(conf: ::config::Config) -> Result<Config, Error> {
        Ok(Config {
            discord_token: conf.get_str("discord_token").map_err(|_| ::failure::err_msg("No discord token found"))?,
            name: conf.get_str("name").map_err(|_| ::failure::err_msg("Not Bot name given"))?,
            cmd_char: conf.get_str("cmd_char").map_err(|_| ::failure::err_msg("Not command char given"))?,
        })
    }
}