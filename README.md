# Join Alerts

Gateway service for relaying MEMBER_JOIN events.

Takes `DISCORD_TOKEN` and `TARGET_URI` environment variables, connects a single shard to the gateway, and forwards all MEMBER_JOIN events to the target URI.

## Payload
```json
{
  "guild_id": 123456789123456789,
  "user_id": 123456789123456789
}
```