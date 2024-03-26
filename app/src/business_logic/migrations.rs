use sqlx::sqlite::SqlitePool;
use sqlx::Error;

pub async fn migrate(pool: &SqlitePool) -> Result<(), Error> {
    create_users(pool).await?;
    create_posts(pool).await?;
    Ok(())
}

async fn create_users(pool: &SqlitePool) -> Result<(), Error> {
    sqlx::query(
        "
    CREATE TABLE users (
        user_id TEXT PRIMARY KEY,
        identity_id TEXT NOT NULL,
        email TEXT NOT NULL
    );
    CREATE INDEX IF NOT EXISTS idx_identity_id ON users (identity_id);",
    )
    .execute(pool)
    .await?;
    Ok(())
}

async fn create_posts(pool: &SqlitePool) -> Result<(), Error> {
    sqlx::query(
        "
    CREATE TABLE IF NOT EXISTS posts (
        post_id TEXT PRIMARY KEY,
        user_id TEXT NOT NULL,
        content TEXT NOT NULL,
        FOREIGN KEY (user_id) REFERENCES users(user_id)
    );",
    )
    .execute(pool)
    .await?;
    Ok(())
}
