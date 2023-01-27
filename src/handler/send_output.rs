use serenity::{prelude::Context, model::Timestamp};
use tokio::{io::{Lines, BufReader}, process::{ChildStdout, ChildStderr}};

use crate::{OutputModes, ChannelList};


pub async fn send_output(ctx: &Context, mode: &OutputModes, mut stdout_reader: Lines<BufReader<ChildStdout>>, mut stderr_reader: Lines<BufReader<ChildStderr>>) {
    match mode {
        OutputModes::STDOUT => {
            while let Some(line) = stdout_reader.next_line().await.unwrap() {
                for channel in ctx.data.read().await.get::<ChannelList>().unwrap().iter() {

                    if !line.is_empty() {
                        if let Err(why) = channel.send_message(&ctx.http, |m| {
                            m.embed(|e| {
                                e.description(&line)
                                    .timestamp(Timestamp::now())
                            })
                        }).await {
                            println!("Error sending message: {:?}", why);
                        }
                    }
                }
            }
        },
        OutputModes::STDERR => {
            while let Some(line) = stderr_reader.next_line().await.unwrap() {
                for channel in ctx.data.read().await.get::<ChannelList>().unwrap().iter() {

                    if !line.is_empty() {
                        if let Err(why) = channel.send_message(&ctx.http, |m| {
                            m.embed(|e| {
                                e.description(&line)
                                    .timestamp(Timestamp::now())
                            })
                        }).await {
                            println!("Error sending message: {:?}", why);
                        }
                    }
                }
            }
        } 
    }
}
