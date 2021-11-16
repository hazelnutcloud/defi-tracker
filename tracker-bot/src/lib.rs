use anyhow::Result;
use teloxide::{ prelude::*, utils::command::BotCommand };
use defi_tracker::fantom::tomb_finance;

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

pub async fn formatted_stats() -> Result<String> {
    let wallet_address = "0x08d6A1d7f3715f442e8e9dbe80CB6f0139c2735e";
    let stats = tomb_finance::stats(wallet_address).await?;

    Ok(format!("\
Wallet Address: {wallet_address}

----------------------------------------
MASONRY
----------------------------------------
Unclaimed rewards: {rewards_masonry} (${rewards_masonry_usd})
Total staked: {staked_masonry} (${staked_masonry_usd})

----------------------------------------
TOMB-FTM CEMETERY
----------------------------------------
Unclaimed rewards: {rewards_tombftm} (${rewards_tombftm_usd})
Total staked: {staked_tombftm} (${staked_tombftm_usd})

----------------------------------------
TSHARE-FTM CEMETERY
----------------------------------------
Unclaimed rewards: {rewards_tshareftm} (${rewards_tshareftm_usd})
Total staked: {staked_shareftm} (${staked_shareftm_usd})

----------------------------------------
TOTAL REWARDS = ${total_rewards}
TOTAL VALUE = ${total_value}
----------------------------------------
",
        wallet_address = wallet_address,
        rewards_masonry = stats.rewards_masonry,
        rewards_masonry_usd = stats.rewardsusd_masonry,
        staked_masonry = stats.staked_masonry,
        staked_masonry_usd = stats.stakedusd_masonry,
        rewards_tombftm = stats.rewards_tombftm,
        rewards_tombftm_usd = stats.rewards_tombftm_usd,
        staked_tombftm = stats.staked_tombftm,
        staked_tombftm_usd = stats.staked_tombftm_usd,
        rewards_tshareftm = stats.rewards_tshareftm,
        rewards_tshareftm_usd = stats.rewards_tshareftm_usd,
        staked_shareftm = stats.staked_tshareftm,
        staked_shareftm_usd = stats.staked_tshareftm_usd,
        total_value = stats.total_value,
        total_rewards = stats.total_rewards,
    ))
}