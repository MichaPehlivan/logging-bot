use std::process::{Stdio};

use serenity::{prelude::*, async_trait, model::{prelude::{Ready, Message, Activity}}};
use tokio::{process::Command, io::{BufReader, AsyncBufReadExt}, time::{sleep, Duration}};

use crate::{ChannelList, CommandData, OutputModes};

mod send_output;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        ctx.set_activity(Activity::playing(&ctx.data.read().await.get::<CommandData>().unwrap().args[0..2][0])).await; //sets activity to script name

        while ctx.data.read().await.get::<ChannelList>().unwrap().is_empty() {
            sleep(Duration::from_micros(1)).await;
        }

        let ctx_data_clone = ctx.data.clone();
        let ctx_data = ctx_data_clone.read().await;
        let program_args = ctx_data.get::<CommandData>().unwrap();

        let mut cmd = Command::new(program_args.shell.program())
                                .current_dir(&program_args.dir) //needed for script context
                                .args(&program_args.args)
                                .stdout(Stdio::piped())
                                .stderr(Stdio::piped())
                                .spawn()
                                .expect("unable to spawn program");

        let stdout = cmd.stdout.take().expect("command did not have handle to stdout");
        let stdout_reader = BufReader::new(stdout).lines();
        let stderr = cmd.stderr.take().expect("command did not have handle to stderr");
        let stderr_reader = BufReader::new(stderr).lines();

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

        let data_clone = ctx_clone.data.read().await;
        let mode = data_clone.get::<OutputModes>().unwrap();

        send_output::send_output(&ctx_clone, mode, stdout_reader, stderr_reader, ctx_clone.data.read().await.get::<ChannelList>().unwrap()).await;
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
