mod models;
mod manager;
mod controller;
mod commands;
mod bot;
mod repo;
use poise::serenity_prelude as serenity;
use tokio::runtime::Builder;
use crate::{bot::Bot,commands::{check, exec, handle_event, list, start, stop},models::Error, repo::get_ctfs};

async fn async_main()->Result<(),Error> {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let data:Bot = Bot::new(get_ctfs()).await?;

    println!("Created bot");
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![list(),stop(),start(),exec(),check()],
            event_handler:|_ctx,_event,_framework,_bot|{
                Box::pin(handle_event(_ctx, _event, _framework,_bot))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(data)
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
    Ok(())
}


fn main()->Result<(),Error>{
 let runtime = Builder::new_current_thread()
        .enable_all() // Enable all I/O features like time, net, etc.
        .build()?;
    
    // Block the thread until the async main completes
    runtime.block_on(async_main())
}