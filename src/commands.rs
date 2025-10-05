
use crate::{bot::Bot,models::Error};
use poise::serenity_prelude as serenity;
type Context<'a> = poise::Context<'a, Bot, Error>;


#[poise::command(slash_command,guild_only)]
pub async fn list(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let bot = ctx.data();
    let guild_id = ctx.guild().unwrap().id.to_string();
    let m =bot.man_mut(&guild_id).await;
    let man = m.lock().await;
    let response = man.list_ctf();
    ctx.say(response).await?;
    Ok(())
}


#[poise::command(slash_command,guild_only)]
pub async fn start(
    ctx: Context<'_>,
    #[description= "CTF ID"] ctf_id:String
) -> Result<(), Error> {
    let bot = ctx.data();
    let guild_id = ctx.guild().unwrap().id.to_string();
    let user_id = ctx.author().id.to_string();
    let user = bot.valid_usr(&guild_id, &user_id).await;
    if user.ongoing {
        return Err("A ctf is already ongoing".into())
    }
    let ctf = match bot.ctfs.iter().find(|ctf| ctf.id == ctf_id) {
        None=>return Err(format!("Unrecognized CTF: {}", ctf_id).into()),
        Some(ctf)=>ctf
    };
    ctx.defer().await?; 
    let pod_name = bot.control.create_job(&user,ctf).await?;
    let sess = bot.control.start_session(&user, ctf,&pod_name).await?;
    let m = bot.man_mut(&guild_id).await;
    let mut man = m.lock().await;
    man.add_session(sess);
    let response = "Added session".to_string();
    ctx.say(response).await?;
    Ok(())
}


#[poise::command(slash_command,guild_only)]
pub async fn stop(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let bot = ctx.data();
    let guild_id = ctx.guild().unwrap().id.to_string();
    let user_id = ctx.author().id.to_string();
    let user = bot.valid_usr(&guild_id, &user_id).await;
    if !user.ongoing {
        return Err("No ongoing ctf to stop".into())
    }
    let m = bot.man_mut(&guild_id).await;
    let mut man = m.lock().await;
    ctx.defer().await?; 
    man.remove_session(&user);
    bot.control.delete_job(&user).await?;
    ctx.say("Stopped session").await?;
    Ok(())
}


#[poise::command(slash_command,guild_only)]
pub async fn exec(
    ctx: Context<'_>,
    #[description="Command to execute"] cmd:String
) -> Result<(), Error> {
    let bot = ctx.data();
    let guild_id = ctx.guild().unwrap().id.to_string();
    let user_id = ctx.author().id.to_string();
    let user = bot.valid_usr(&guild_id, &user_id).await;
    if !user.ongoing {
        return Err("No ongoing ctf to execute".into())
    }
    let m = bot.man_mut(&guild_id).await;
    let mut man = m.lock().await;
    ctx.defer().await?; 
    let output = man.write_to_session(&user, &cmd).await?;
    if output.len() == 0{
        ctx.say("No output.").await?;
    } else {
        ctx.say(output).await?;
    }
    Ok(())
}


#[poise::command(slash_command,guild_only)]
pub async fn check(
    ctx: Context<'_>,
    #[description="Flag to check"] flag:String
) -> Result<(), Error> {
    
    let bot = ctx.data();
    let guild_id = ctx.guild().unwrap().id.to_string();
    let user_id = ctx.author().id.to_string();
    let user = bot.valid_usr(&guild_id, &user_id).await;
    if !user.ongoing {
        return Err(format!("No ongoing ctf to check").into())
    }
    let m = bot.man_mut(&guild_id).await;
    let man = m.lock().await;
    let text =  match man.check_flag(&flag, &user) {
        true=>"Congratulations, you found the flag ! ",
        false=>"Where did you get this one from buddy? Are we just trying random things?"
    };
    ctx.say(text).await?;
    Ok(())
}

pub async fn handle_event(
    _: &serenity::Context,
    event: &serenity::FullEvent,
    _: poise::FrameworkContext<'_, Bot, Error>,
    bot:&Bot
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::GuildCreate{ guild, is_new: _} => {
            let guild_id = guild.id.to_string();
            if !bot.man_exists(&guild_id).await{
                bot.add_manager(guild_id).await;
            }
        }
        _ => {}
    }
    Ok(())
}
