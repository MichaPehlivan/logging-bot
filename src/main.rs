use std::env;

use serenity::{prelude::*, model::prelude::ChannelId};

mod handler;

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
        match shell {
            ".sh" => Shell::BASH,
            ".bat" => Shell::CMD,
            ".cmd" => Shell::CMD,
            _=> panic!("filetype not supported")
        }
    }

    fn program(&self) -> &str {
        match self {
            Shell::BASH => "bash",
            Shell::CMD => "cmd",
        }
    }

    fn args(&self) -> String {
        match self {
            Shell::BASH => "".to_string(),
            Shell::CMD => "/C".to_string(),
        }
    }
}

struct CommandData {
    shell: Shell,
    dir: String,
    args: Vec<String>
}

impl TypeMapKey for CommandData {
    type Value = CommandData;
}

fn parse_arg(arg: &String) -> CommandData {
    let filename_index = arg.rfind("/").unwrap() + 1;
    let file_extension = &arg[arg.rfind(".").unwrap()..];
    let shell = Shell::from_str(file_extension);
    
    CommandData { 
        dir: arg[..filename_index].to_string(), 
        args: {
            let file_arg = arg[filename_index..].to_string();
            let shell_args = shell.args();

            if shell_args != "" { vec![shell_args, file_arg] } else { vec![file_arg] }
        },
        shell: shell,
    }
}

#[tokio::main]
async fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let token = env::var("BOT_TOKEN").expect("token not found");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents).event_handler(handler::Handler).await.expect("Error creating client");
    {
        let mut data = client.data.write().await;
        data.insert::<CommandData>(parse_arg(args.get(0).unwrap()));
        data.insert::<ChannelList>(Vec::new());
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
