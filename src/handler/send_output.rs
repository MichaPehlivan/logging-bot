use serenity::{prelude::Context, model::{prelude::ChannelId, Timestamp}};
use tokio::{io::{Lines, BufReader}, process::{ChildStdout, ChildStderr}};

use crate::OutputModes;


pub async fn send_output(ctx: &Context, mode: &OutputModes, mut stdout_reader: Lines<BufReader<ChildStdout>>, mut stderr_reader: Lines<BufReader<ChildStderr>>, channels: &Vec<ChannelId>) {
    match mode {
        OutputModes::STDOUT => {
            while let Some(line) = stdout_reader.next_line().await.unwrap() {
                for channel in channels.iter() {

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
                for channel in channels.iter() {

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
