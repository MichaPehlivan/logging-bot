use std::env;

use serenity::{prelude::*, model::prelude::ChannelId};

mod handler;

struct ProgramArgs;

impl TypeMapKey for ProgramArgs {
    type Value = CommandData;
}

struct ChannelList;

impl TypeMapKey for ChannelList {
    type Value = Vec<ChannelId>;
}

#[derive(Clone)]
enum Shell {
    BASH,
    CMD
}

impl Shell {
    fn from_str(shell: &str) -> Shell {
        if shell == ".sh" {
            return Shell::BASH;
        }
        else if shell == ".bat" || shell == ".cmd" {
            return Shell::CMD;
        }
        else {
            panic!("filetype not supported")
        } 
    }

    fn program(&self) -> &str {
        match self {
            Shell::BASH => "bash",
            Shell::CMD => "cmd",
        }
    }

    fn args(&self) -> &str {
        match self {
            Shell::BASH => "",
            Shell::CMD => "/C",
        }
    }
}

struct CommandData {
    shell: Shell,
    dir: String,
    args: Vec<String>
}

fn parse_arg(arg: &String) -> CommandData {
    let index = arg.rfind("/").unwrap();
    let arg_as_str = arg.as_str();
    let file_extension = &arg_as_str[arg.rfind(".").unwrap()..];
    let shell = Shell::from_str(file_extension);
    
    CommandData { 
        shell: shell.clone(),
        dir: arg_as_str[..index].to_string(), 
        args: {
            if shell.args().to_string() != "" {
                vec![shell.args().to_string(), arg_as_str[index+1..].to_string()]
            }
            else {
                vec![arg_as_str[index+1..].to_string()]
            }
        } 
    }
}

#[tokio::main]
async fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let token = env::var("BOT_TOKEN").expect("token not found");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents).event_handler(handler::Handler).await.expect("Error creating client");
    {
        let mut data = client.data.write().await;
        data.insert::<ProgramArgs>(parse_arg(args.get(0).unwrap()));
        data.insert::<ChannelList>(Vec::new());
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
