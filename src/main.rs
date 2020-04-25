use serenity::prelude::*;
use serenity::Client;

mod conf;
mod imgur;
use crate::imgur::Imgur;
use conf::Config;
use rand::rngs::OsRng;
use serenity::framework::standard::macros::{command, group, help};
use serenity::framework::standard::{
    help_commands::*, Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::framework::StandardFramework;
use serenity::model::channel::Message;
use serenity::model::prelude::*;
use std::collections::HashSet;

#[group]
#[commands(pic)]
struct General;
struct Handler;

impl EventHandler for Handler {}

impl TypeMapKey for Config {
    type Value = Config;
}

impl TypeMapKey for Imgur {
    type Value = Imgur;
}

fn main() {
    let conf = Config::new().unwrap();
    let mut client = Client::new(conf.discord_token.unsecure(), Handler).unwrap();

    let imgur = Imgur::new(&conf).unwrap();
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("~"))
            .help(&MY_HELP)
            .group(&GENERAL_GROUP),
    );
    {
        let mut data = client.data.write();
        data.insert::<Config>(conf);
        data.insert::<Imgur>(imgur);
    }

    if let Err(e) = client.start() {
        eprintln!("{}", e);
    }
}

#[command]
fn pic(ctx: &mut Context, msg: &Message) -> CommandResult {
    let query = msg.content[4..].trim();
    let image = {
        let data = ctx.data.read();
        data.get::<Imgur>().unwrap().get_rand(if query.is_empty() {"waluigi"} else {query}, &mut OsRng::default())
    };

    match image {
        Ok(url) => msg.reply(ctx, url),
        Err(e) => msg.reply(ctx, format!("waaaah {}", e))
    }?;

    Ok(())
}

#[help]
fn my_help(
    context: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    with_embeds(context, msg, args, &help_options, groups, owners)
}
