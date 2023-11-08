use crate::db;
use crate::models::Expense;
use chrono;
use sqlx::PgPool;
use teloxide::RequestError;
use teloxide::{prelude::*, utils::command::BotCommands};
use speedate::Date;


#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "add an expense.", parse_with = "split")]
    Check {price: f64, comments: String },
    #[command(description = "get statistics.", parse_with = "split")]
    Stat { date_start: String, date_end: String },
    #[command(description = "start")]
    Start,
}

pub async fn handle_command(
    bot: Bot,
    command: Command, 
    pool: PgPool, 
    user_id: i64, 
    username: String,
    chat_id: ChatId,
) -> Result<(), RequestError> {
    match command {
        Command::Check { price, comments} => {
            let expense = Expense {                     
                user_id,
                username: username.clone(),
                price,
                comments,
                date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
            };

            match db::add_expense(&pool, &expense).await {
                Ok(_) => println!("Expense added"),
                Err(e) => eprintln!("Error adding expense: {}", e),
            }

            bot.send_message(chat_id, &format!("Expense added for {}", username)).await.log_on_error().await;
        },
        Command::Stat {date_start, date_end}  => {
            match (Date::parse_str(&date_start), Date::parse_str(&date_end)) {
                (Ok(date_start), Ok(date_end)) => {
                    match db::get_expenses_by_date(&pool, date_start, date_end).await {
                        Ok(expenses) => {
                            let mut totals: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
                            for exp in expenses {
                                *totals.entry(exp.username.clone()).or_insert(0.0) += exp.price;
                            }
                            let mut message = String::from("Expense Summary:\n");
                            for (username, total) in totals {
                                message.push_str(&format!("{}: {:.2}\n", username, total));
                            }
                            bot.send_message(chat_id, message).await.log_on_error().await;
                        },
                        Err(e) => {
                            log::error!("Error getting expenses: {}", e);
                            bot.send_message(chat_id, "An error occured").await.log_on_error().await;
                        }
                    }
                }
                (Err(e), _)  => {
                    log::error!("{}", e);
                    bot.send_message(chat_id, "Wrong start date format").await.log_on_error().await;
                }
                (_, Err(e))  => {
                    log::error!("{}", e);
                    bot.send_message(chat_id, "Wrong end date format").await.log_on_error().await;
                }
            }
        },
        Command::Start => {
            bot.send_message(chat_id, "Hello!").await.log_on_error().await;
        }
    }

    Ok(())
}
