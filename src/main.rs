use futures_util::StreamExt;
use serde_derive::Serialize;
use std::{env, error::Error, sync::Arc};
use twilight_gateway::cluster::ClusterBuilder;
use twilight_gateway::{Event, Intents};

use reqwest::Client;

#[derive(Serialize)]
struct MemberData {
    pub guild_id: String,
    pub member_id: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let token: String = env::var("DISCORD_TOKEN")?;
    let intents: Intents = Intents::GUILD_MEMBERS;

    let (cluster, mut events) = ClusterBuilder::new(token, intents).build().await?;

    let cluster = Arc::new(cluster);
    cluster.up().await;

    let target = env::var("TARGET_URI")?;
    let client = reqwest::Client::new();

    while let Some((shard_id, event)) = events.next().await {
        tokio::spawn(handle_event(
            shard_id,
            event,
            client.clone(),
            target.clone(),
        ));
    }

    Ok(())
}

async fn handle_event(shard_id: u64, event: Event, client: Client, target: String) {
    match event {
        Event::MemberAdd(member) => {
            let member_data = MemberData {
                guild_id: member.guild_id.to_string(),
                member_id: member.user.id.to_string(),
            };

            let res = client.post(&target).json(&member_data).send().await;

            match res {
                Ok(res) => {
                    if res.status() != 200 {
                        println!(
                            "Unexpected Result on posting Member Join event: {}",
                            res.status()
                        );
                    }
                }
                Err(e) => println!("Error on posting Member Join event: {}", e),
            }
        }

        Event::ShardConnected(_) => {
            println!("Shard #{} ready!", shard_id);
        }

        Event::ShardDisconnected(_) => {
            println!("Shard #{} Disconnected!", shard_id);
        }

        Event::Resumed => {
            println!("Shard #{} resumed.", shard_id);
        }

        _ => {}
    }
}
