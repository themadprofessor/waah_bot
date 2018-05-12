#![feature(process_exitcode_placeholder)]

#[macro_use] extern crate serenity;
extern crate config;
extern crate directories;
extern crate failure;

use serenity::prelude::*;
use serenity::{
    Client,
    model::channel::Message
};
use failure::Error;

mod conf;

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content.as_bytes()[0] == '~' as u8 {
            if msg.content == "~ping" {
                msg.reply("Pong").unwrap();
            }
        }
    }
}

fn run() -> Result<(), Error> {
    let conf = {
        let project_dirs = ::directories::ProjectDirs::from("io", "waluigi", "waah_bot");
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
        #[cfg(debug_assertions)] config.merge(::config::File::with_name("token.toml"))?; //Only include token.toml if this a test scenario
        conf::Config::from_conf(config)
    }?;
    println!("Config loaded");

    let mut client = Client::new(
        &conf.discord_token,
        Handler).map_err(|e| ::failure::err_msg(format!("{}", e)))?;
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
