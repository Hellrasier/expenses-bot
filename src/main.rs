mod bot;
mod commands;
mod db;
mod models;
use sqlx::PgPool;
use shuttle_secrets::SecretStore;
use bot::BotService;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_secrets::Secrets] secrets: SecretStore,
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> Result<BotService, shuttle_runtime::Error> {

    let _ = sqlx::migrate!().run(&pool).await.map_err(|e| format!("Oh no! Migrations failed :( {e}"));

    let token = secrets.get("BOT_TOKEN")
        .expect("Bot token is required");

    Ok(BotService::new(token, pool))
}


#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for BotService {
    async fn bind(self, _addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        self.run().await
            .expect("An error occured when starting the bot");
        Ok(())
    }
}
