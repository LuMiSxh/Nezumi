# Nezumi - The new Discord Bot

## Setup

Create a `.env` file containing the following keys:
- `bot_token` containing the Discord bot token

Change the settings in `Settings.toml` to your desire.


## Start

To start the bot, either install SurrealDB and run it locally (`surreal start`) and start the bot via 
`cargo build --release` and `cargo run`. The alternative is running the `docker-compose.yml` file, which
sets up a SurrealDb instance and the Discord bot project.
