use futures_util::StreamExt;
use serde_derive::Serialize;
use std::{env, error::Error};
use twilight_gateway::cluster::{ClusterBuilder, ShardScheme};
use twilight_gateway::{Event, Intents, Shard};

#[derive(Serialize)]
struct MemberData {
    pub guild_id: String,
    pub member_id: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let (cluster, mut events) =
        ClusterBuilder::new(env::var("DISCORD_TOKEN"), Intents::GUILD_MEMBERS)
            .build()
            .await?;

    let target = env::var("TARGET_URI")?;
    let client = reqwest::Client::new();

    while let Some(event) = events.next().await {
        match event {
            Event::MemberAdd(member) => {
                let member_data = MemberData {
                    guild_id: member.guild_id.to_string(),
                    member_id: member.user.id.to_string(),
                };
                let json = serde_json::to_string(&member_data).unwrap();
                let res = client.post(&target).body(json).send().await?;
                println!("{:?}", res);
            }

            Event::ShardConnected(shard) => {
                println!("Shard #{} ready!", shard.id);
            }

            Event::ShardDisconnected(shard) => {
                println!("Shard #{} Disconnected!", shard.id);
            }

            Event::Resumed(shard) => {
                println!("Shard #{} resumed.", shard.id);
            }
        }

        if let Event::MemberAdd(member) = event {
            let data = MemberData {
                guild_id: member.guild_id.to_string(),
                member_id: member.user.id.to_string(),
            };

            client.post(&target).json(&data).send().await?;

            continue;
        }

        events::handle_gateway_event(id, event)
    }

    Ok(())
}
