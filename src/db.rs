use sqlx::{PgPool, query, query_as};
use crate::models::Expense;

pub async fn add_expense(pool: &PgPool, expense: &Expense) -> Result<u64, sqlx::Error> {
    let result = query("INSERT INTO expenses (user_id, username, price, comments, date) VALUES ($1, $2, $3, $4, $5)")
        .bind(expense.user_id)
        .bind(&expense.username)
        .bind(expense.price)
        .bind(&expense.comments)
        .bind(&expense.date)
        .execute(pool)
        .await?;

    // PostgreSQL does not support `last_insert_rowid()`
    // Instead, return the result of `execute` which is the number of rows inserted
    Ok(result.rows_affected())
}

pub async fn get_expenses_by_date(pool: &PgPool, date_start: &str, date_end: &str) -> Result<Vec<Expense>, sqlx::Error> {
    let expenses = query_as::<_, Expense>(
        "SELECT user_id, username, price, comments, date FROM expenses WHERE date BETWEEN $1 AND $2"
    )
    .bind(date_start)
    .bind(date_end)
    .fetch_all(pool)
    .await?;

    Ok(expenses)
}