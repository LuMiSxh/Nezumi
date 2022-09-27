use poise::serenity_prelude as serenity;
use std::time::Duration;
use common::configuration::SETTINGS;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
// Nutzer Daten, welche gespeichert und in allen Befehlen zu Verfügung stehen
struct Data {}

// Eigener Error Handler
async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Bot konnte nicht gestartet werden: {:?}", error),
        poise::FrameworkError::Command { error, ctx } => {
            println!("Fehler in Befehl `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Fehler während der Behebung von Fehler: {}", e)
            }
        }
    }
}

pub async fn run_discord_bot() {
    let options = poise::FrameworkOptions {
        // Befehle
        commands: vec![
        ],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some(SETTINGS.read().unwrap().get::<String>("Bot_Prefix").unwrap()),
            edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(3600))),
            additional_prefixes: vec![
                poise::Prefix::Literal("Mouse"),
                poise::Prefix::Literal("mouse"),
            ],
            ..Default::default()
        },
        // Globaler Error Handler
        on_error: |error| Box::pin(on_error(error)),
        // Läuft vor jedem Befehl
        pre_command: |_ctx| {
            Box::pin(async move {
                // println!("Starte Befehl: {}...", ctx.command().qualified_name);
            })
        },
        // Läuft nach jedem Befehl, wenn er OK() zurückgibt
        post_command: |_ctx| {
            Box::pin(async move {
                // println!("Folgenden Befehl ausgeführt: {}", ctx.command().qualified_name);
            })
        },
        // Check den jeder Befehl durchlaufen muss
        command_check: Some(|_ctx| {
            Box::pin(async move {
               Ok(true)
            })
        }),
        // Wird vor jedem Event ausgelöst
        listener: |_ctx, _event, _framework, _data| {
            Box::pin(async move {
                // println!("Event in Zuhörer: {:?}", event.name());
                Ok(())
            })
        },
        ..Default::default()
    };

    poise::Framework::builder()
        .token(SETTINGS.read().unwrap().get::<String>("Bot_Prefix").unwrap())
        .user_data_setup(move |_ctx, _ready, _framework| {
            Box::pin(async move { Ok(Data {}) })
        })
        .options(options)
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .run()
        .await
        .unwrap();
}