# Guild Counter

A standalone service for keeping track of a Discord bot's guild count. 

Takes a single `DISCORD_TOKEN` environment variable, port and listening address can be configured in `Rocket.toml`. Default Port: 9080.

## GET /
```json
{
  "count": 69
}
```
