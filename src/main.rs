#![feature(process_exitcode_placeholder)]

extern crate serenity;
extern crate config;
extern crate directories;
extern crate failure;
extern crate reqwest;
extern crate serde;
extern crate rand;
#[macro_use] extern crate serde_derive;

use serenity::prelude::*;
use serenity::{
    Client,
    model::channel::Message
};
use failure::Error;

mod conf;
mod cmd;
mod imgur;

struct Handler {
    conf: conf::Config,
    imgur: imgur::Imgur
}

impl Handler {
    pub fn new(conf: conf::Config) -> Handler {
        let imgur = imgur::Imgur::new(&conf).unwrap();
        Handler {conf, imgur}
    }
}

impl EventHandler for Handler {
    fn message(&self, _ctx: Context, msg: Message) {
        if msg.content.starts_with(&self.conf.cmd_char) {
            println!("Command recieved");
            let mut split = msg.content[self.conf.cmd_char.len()..].split(' ');
            match &split.next() {
                Some(cmd) => {
                    match *cmd {
                        "ping" => ::cmd::ping(&msg),
                        "wah" => ::cmd::wah(&msg, split),
                        "img" => ::cmd::img(&msg, &self.imgur),
                        "help" => ::cmd::help(&msg),
                        _ => {}
                    }
                },
                None => ()
            }
        }
    }
}

fn run() -> Result<(), Error> {
    let conf = {
        let project_dirs = ::directories::ProjectDirs::from("io", "discord", "waah_bot");
        let mut config = ::config::Config::new();
        let paths = [project_dirs.data_dir(),
            project_dirs.data_local_dir(),
            project_dirs.config_dir()];

        for &path in paths.iter() {
            if path.exists() {
                config.merge(config::File::with_name(path.to_str().unwrap()))?;
            }
        }
        config.merge(::config::Environment::with_prefix("waah"))?;
        #[cfg(debug_assertions)] config.merge(::config::File::with_name("debug.toml"))?; //Only include debug.toml if this a test scenario
        conf::Config::from_conf(config)
    }?;
    println!("Config loaded");

    let mut client = Client::new(
        &conf.discord_token.clone(),
        Handler::new(conf)
    ).map_err(|e| ::failure::err_msg(format!("{}", e)))?;
    println!("Client starting");
    if let Err(e) = client.start() {
        eprintln!("Failed to start client! {}", e)
    }
    Ok(())
}

fn main() -> ::std::process::ExitCode {
    match run() {
        Ok(_) => ::std::process::ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{}", e);
            ::std::process::ExitCode::FAILURE
        }
    }
}
