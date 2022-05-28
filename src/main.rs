#![feature(proc_macro_hygiene, decl_macro)]

use futures_util::StreamExt;
use rocket::{get, ignite, routes, State};
use rocket_contrib::json::JsonValue;
use std::{
    collections::HashSet,
    env,
    error::Error,
    sync::{Arc, Mutex},
};
use twilight_gateway::{Event, Intents, Shard};

#[macro_use]
extern crate rocket_contrib;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let (mut events, _shard) = {
        let token = env::var("DISCORD_TOKEN")?;

        let intents = Intents::GUILDS;
        let (shard, events) = Shard::new(token, intents).await?;

        shard.start().await?;

        (events, shard)
    };

    let mut guilds: HashSet<u64> = HashSet::new();
    let state: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));

    let guild_count_mutex = state.clone();

    tokio::task::spawn_blocking(move || {
        ignite()
            .mount("/", routes![get_guild_count])
            .manage(state.clone())
            .launch();
    });

    while let Some(event) = events.next().await {
        if let Event::GuildCreate(guild) = event {
            guilds.insert(u64::from(guild.id));
            *guild_count_mutex.lock().unwrap() = guilds.len() as u64;

            continue;
        }

        if let Event::GuildDelete(guild) = event {
            guilds.remove(&u64::from(guild.id));
            *guild_count_mutex.lock().unwrap() = guilds.len() as u64;

            continue;
        }
    }

    Ok(())
}

#[get("/")]
fn get_guild_count(guilds: State<Arc<Mutex<u64>>>) -> JsonValue {
    return json!({
      "count": *guilds.lock().unwrap()
    });
}
