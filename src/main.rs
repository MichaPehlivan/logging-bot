use std::{env, process::Stdio};

use serenity::{prelude::*, async_trait, model::prelude::Ready};
use tokio::{process::Command, io::{BufReader, AsyncBufReadExt}};

struct ProgramArgs;

impl TypeMapKey for ProgramArgs {
    type Value = Vec<String>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let mut cmd = Command::new("bash")
                                .args(ctx.data.read().await.get::<ProgramArgs>().unwrap())
                                .stdout(Stdio::piped())
                                .spawn()
                                .expect("unable to spawn program");

        let stdout = cmd.stdout.take().expect("command did not have handle to stdout");
        let mut reader = BufReader::new(stdout).lines();

        tokio::spawn(async move {
            let status = cmd.wait().await
                .expect("process encountered an error");

            println!("process exit status was: {}", status);
        });

        while let Some(line) = reader.next_line().await.unwrap() {
            println!("{}", line);
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

    let mut client = Client::builder(token, intents).event_handler(Handler).await.expect("Error creating client");
    {
        let mut data = client.data.write().await;
        data.insert::<ProgramArgs>(args);
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
