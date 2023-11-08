use chrono::NaiveDate;
use sqlx::{PgPool, query, query_as};
use crate::models::Expense;

pub async fn add_expense(pool: &PgPool, expense: &Expense) -> Result<u64, sqlx::Error> {
    let result = query("INSERT INTO new_expenses (user_id, username, price, comments, date) VALUES ($1, $2, $3, $4, $5)")
        .bind(expense.user_id)
        .bind(&expense.username)
        .bind(expense.price)
        .bind(&expense.comments)
        .bind(&expense.date)
        .execute(pool)
        .await?;

    log::info!("Rows affected: {}", result.rows_affected());
    Ok(result.rows_affected())
}

pub async fn get_expenses_by_date(pool: &PgPool, date_start: NaiveDate, date_end: NaiveDate) -> Result<Vec<Expense>, sqlx::Error> 
    query_as::<_, Expense>(
        "SELECT user_id, username, price, comments, date FROM new_expenses WHERE date >= $1 AND date < $2"
    )
    .bind(date_start.to_string())
    .bind(date_end.to_string())
    .fetch_all(pool)
    .await
}