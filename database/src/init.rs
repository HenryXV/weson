use sqlx::SqlitePool;
use std::env;

pub async fn get_database_pool() -> anyhow::Result<SqlitePool> {
    dotenv::dotenv().ok();

    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    log::info!("{:?}", pool);

    create_table(&pool).await?;

    Ok(pool)
}

async fn create_table(pool: &SqlitePool) -> anyhow::Result<()> {
    let mut conn = pool.acquire().await?;

    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS playlists (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        )
    "#
    )
    .execute(&mut *conn)
    .await?;

    Ok(())
}
