use serenity::prelude::*;
use serenity::Client;

mod conf;
mod imgur;
use crate::imgur::Imgur;
use conf::Config;
use fern::Dispatch;
use log::{debug, error, info, warn};
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
    setup_logging(&conf);
    let mut client = Client::new(conf.discord_token.unsecure(), Handler).unwrap();

    let imgur = Imgur::new(&conf).unwrap();
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("~"))
            .help(&MY_HELP)
            .group(&GENERAL_GROUP)
            .before(|_ctx, msg, cmd_name| {
                info!(
                    "command [{}] received from [{}] [{}]",
                    cmd_name, msg.author.id, msg.author.name
                );
                true
            })
            .after(|ctx, msg, cmd_name, res| {
                if let Err(e) = res {
                    msg.reply(ctx, format!("waaaah {}", e.0));
                    warn!("command [{}] failed [{}]", cmd_name, e.0);
                }
            })
            .unrecognised_command(|_ctx, msg, cmd_name| {
                warn!(
                    "invalid command [{}] received from [{}] [{}]",
                    cmd_name, msg.author.id, msg.author.name
                );
            }),
    );
    {
        let mut data = client.data.write();
        data.insert::<Config>(conf);
        data.insert::<Imgur>(imgur);
    }

    if let Err(e) = client.start() {
        error!("{}", e);
    }
}

fn setup_logging(conf: &Config) {
    Dispatch::new()
        .format(|out, msg, rec| {
            out.finish(format_args!("[{}][{}] {}", rec.level(), rec.target(), msg))
        })
        .level(conf.log_level)
        .chain(::std::io::stdout())
        .apply()
        .unwrap();
}

#[command]
#[description = "Get a random image from Imgur which matches the given query or waluigi if non is given"]
#[usage = "[query]"]
#[example = ""]
#[example = "wario"]
#[help_available]
fn pic(ctx: &mut Context, msg: &Message) -> CommandResult {
    let query = msg.content[4..].trim();
    let image = {
        let data = ctx.data.read();
        data.get::<Imgur>().unwrap().get_rand(
            if query.is_empty() {
                debug!("no query given");
                "waluigi"
            } else {
                query
            },
            &mut rand::thread_rng(),
        )
    };

    msg.reply(ctx, image?)?;
    Ok(())
}

#[help]
#[command_not_found_text = "Waaaaaaaht?"]
#[no_help_available_text = "Waaaaaaaht?"]
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
