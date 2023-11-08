use teloxide::prelude::*;
use sqlx::PgPool;
use crate::commands::{handle_command, Command};
use shuttle_runtime::CustomError;


pub struct BotService {
    pool: PgPool,
    bot: Bot,
}

impl BotService {
    pub fn new(token: String, pool: PgPool) -> Self {
        BotService { pool: pool, bot: Bot::new(token) }
    }

    pub async fn run(&self) -> Result<(), CustomError> {
        log::info!("Starting expense_bot...");

        let bot = self.bot.clone();
        let conn = self.pool.clone();

        Dispatcher::builder(
            bot, 
            Update::filter_message().endpoint(answer),
        )
        .dependencies(dptree::deps![conn]);
        
        Ok(())
    }
}

async fn answer(bot: Bot, conn: PgPool, msg: Message, command: Command) -> ResponseResult<()> {
    let user_id = msg.from().unwrap().id;
    let username = msg.from().unwrap().username.clone().unwrap_or_else(|| "unknown".to_string());
    let chat_id = msg.chat.id;
    handle_command(bot, command, conn, user_id.0, username, chat_id);
    Ok(())
}