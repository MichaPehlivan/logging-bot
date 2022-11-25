use std::{env, process::Stdio};

use serenity::{prelude::*, async_trait, model::prelude::{Ready, ChannelId, Message}};
use tokio::{process::Command, io::{BufReader, AsyncBufReadExt}, time::{sleep, Duration}};

struct ProgramArgs;

impl TypeMapKey for ProgramArgs {
    type Value = Vec<String>;
}

struct ChannelList;

impl TypeMapKey for ChannelList {
    type Value = Vec<ChannelId>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        while ctx.data.read().await.get::<ChannelList>().unwrap().is_empty() {
            sleep(Duration::from_micros(1)).await;
        }

        let mut cmd = Command::new("bash")
                                .args(ctx.data.read().await.get::<ProgramArgs>().unwrap())
                                .stdout(Stdio::piped())
                                .spawn()
                                .expect("unable to spawn program");

        let stdout = cmd.stdout.take().expect("command did not have handle to stdout");
        let mut reader = BufReader::new(stdout).lines();

        let ctx_clone = ctx.clone();
        tokio::spawn(async move {
            let status = cmd.wait().await
                .expect("process encountered an error");

            println!("process exit status was: {}", status);
            
            for channel in ctx.data.read().await.get::<ChannelList>().unwrap().iter() {

                if let Err(why) = channel.say(&ctx.http, format!("process exited with status code {}", status)).await {
                    println!("Error sending message: {:?}", why);
                }
            }
        });

        while let Some(line) = reader.next_line().await.unwrap() {
            for channel in ctx_clone.data.read().await.get::<ChannelList>().unwrap().iter() {

                if let Err(why) = channel.say(&ctx_clone.http, &line).await {
                    println!("Error sending message: {:?}", why);
                }
            }
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!log" {
            let mut data = ctx.data.write().await;
            data.get_mut::<ChannelList>().unwrap().push(msg.channel_id);
            
            if let Err(why) = msg.channel_id.say(&ctx.http, "now posting logs in this channel").await {
                println!("Error sending message: {:?}", why);
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

    let mut client = Client::builder(token, intents).event_handler(Handler).await.expect("Error creating client");
    {
        let mut data = client.data.write().await;
        data.insert::<ProgramArgs>(args);
        data.insert::<ChannelList>(Vec::new());
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
