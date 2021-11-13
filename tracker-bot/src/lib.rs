use anyhow::Result;
use teloxide::{ prelude::*, utils::command::BotCommand };
use fantom_tracker::tomb_finance;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported: ")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "view your stats.")]
    Stats
}

pub async fn run() -> Result<()> {
    teloxide::enable_logging!();
    log::info!("Starting tomb_tracker_bot...");

    let bot = Bot::from_env().auto_send();

    let bot_name: String = "tomb tracker bot".to_string();
    teloxide::commands_repl(bot, bot_name, answer).await;
    Ok(())
}

async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<()> {
    match command {
        Command::Help => cx.answer(Command::descriptions()).await?,
        Command::Stats => cx.answer(formatted_stats().await?).await?,
    };

    Ok(())
}

async fn formatted_stats() -> Result<String> {
    let stats = tomb_finance::stats("0x08d6A1d7f3715f442e8e9dbe80CB6f0139c2735e").await?;

    Ok(format!("MASONRY
--------------------------
Unclaimed rewards: {} (${})
Total staked: {} (${})",
        stats.rewards_masonry,
        stats.rewardsusd_masonry,
        stats.staked_masonry,
        stats.stakedusd_masonry,
    ))
}