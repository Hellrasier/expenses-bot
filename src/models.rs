#[derive(sqlx::FromRow)]
pub struct Expense {
  pub user_id: i64,
  pub username: String,
  pub price: f32,
  pub comments: String,
  pub date: String,
}