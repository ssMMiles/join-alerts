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
    let token = env::var("DISCORD_TOKEN")?;

    let (cluster, mut events) = ClusterBuilder::new(client.token().unwrap().to_string(), intents)
        .shard_scheme(ShardScheme::try_from((
            (cluster_id * shards_per_cluster..(cluster_id + 1) * shards_per_cluster),
            shards_per_cluster * clusters,
        ))?)
        .presence(UpdatePresencePayload {
            activities: vec![Activity {
                application_id: None,
                assets: None,
                buttons: vec![],
                created_at: None,
                details: None,
                emoji: None,
                flags: None,
                id: None,
                instance: None,
                kind: ActivityType::Watching,
                name: "my shiny new gears turning".to_string(),
                party: None,
                secrets: None,
                state: None,
                timestamps: None,
                url: None,
            }],
            afk: false,
            since: None,
            status: Status::Online,
        })
        .build()
        .await?;

    let (cluster, mut events) = {
        let token = env::var("DISCORD_TOKEN")?;

        let intents = Intents::GUILD_MEMBERS;
        let (shard, events) = Shard::new(token, intents).await?;

        match shard.start().await {
            Ok(_) => println!("Gateway connected."),
            Err(err) => println!("Error: {}", err),
        }

        (events, shard)
    };

    let target = env::var("TARGET_URI")?;
    let client = reqwest::Client::new();

    while let Some(event) = events.next().await {
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
