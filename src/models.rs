#[derive(sqlx::FromRow)]
pub struct Expense {
  pub user_id: i64,
  pub username: String,
  pub price: f64,
  pub comments: String,
  pub date: String,
}
