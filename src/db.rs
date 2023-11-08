use sqlx::{PgPool, query};
use crate::models::Expense;

pub async fn add_expense(pool: PgPool, expense: &Expense) -> Result<i64, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    let result = query!(
        "INSERT INTO expenses (user_id, username, price, comments, date)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        expense.user_id,
        expense.username,
        expense.price,
        expense.comments,
        expense.date
    )
    .execute(&mut conn)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn get_expenses_by_date(pool: PgPool, date_start: &str, date_end: &str) -> Result<Vec<Expense>, sqlx::Error> {
    let mut expenses = Vec::new();
    let mut rows = query!(
        "SELECT user_id, username, price, comments, date FROM expenses
         WHERE date BETWEEN ?1 AND ?2",
        date_start,
        date_end
    )
    .fetch(pool);

    while let Some(row) = rows.try_next().await? {
        expenses.push(Expense {
            user_id: row.user_id,
            username: row.username,
            price: row.price,
            comments: row.comments,
            date: row.date,
        });
    }

    Ok(expenses)
}