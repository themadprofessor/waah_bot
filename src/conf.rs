use failure::Error;

#[derive(Debug, Clone, Hash, PartialEq)]
pub struct Config {
    pub discord_token: String,
    pub cmd_char: String,
    pub imgur_id: String
    //map: HashMap<&'a str, Vec<&'a str>>
}

impl Config {
    pub fn from_conf(conf: ::config::Config) -> Result<Config, Error> {
        Ok(Config {
            discord_token: conf.get_str("discord_token").map_err(|_| ::failure::err_msg("No discord token found"))?,
            cmd_char: conf.get_str("cmd_char").map_err(|_| ::failure::err_msg("No command char given"))?,
            imgur_id: conf.get("imgur_id").map_err(|_| ::failure::err_msg("No imgur id given"))?,
        })
    }
}