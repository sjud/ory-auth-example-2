use ory_kratos_client::models::identity::Identity;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, FromRow};

// This will just map into ServerFnError when we call it in our serverfunctions with ? error handling
use sqlx::Error;
#[tracing::instrument(err)]
pub async fn create_user(pool: &SqlitePool, identity_id: String) -> Result<(), Error> {
    sqlx::query("INSERT INTO users (user_id,identity_id) VALUES (?,?)")
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(identity_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Returns the POST ROW
#[tracing::instrument(err)]
pub async fn create_post(
    pool: &SqlitePool,
    user_id: String,
    content: String,
) -> Result<PostRow, Error> {
    sqlx::query_as::<_, PostRow>(
        "INSERT INTO posts (post_id,user_id,content) VALUES (?,?,?) RETURNING post",
    )
    .bind(uuid::Uuid::new_v4().to_string())
    .bind(user_id)
    .bind(content)
    .fetch_one(pool)
    .await
}
#[tracing::instrument(err)]
pub async fn edit_post(pool: &SqlitePool, post_id: &String, content: &String) -> Result<(), Error> {
    sqlx::query("UPDATE posts SET content = ?  WHERE post_id = ?")
        .bind(post_id)
        .bind(content)
        .execute(pool)
        .await?;
    Ok(())
}
#[tracing::instrument(err)]
pub async fn delete_post(pool: &SqlitePool, post_id: String) -> Result<(), Error> {
    sqlx::query("DELETE FROM posts where post_id = ?")
        .bind(post_id)
        .execute(pool)
        .await?;
    Ok(())
}
#[tracing::instrument(err)]
pub async fn list_users(pool: &SqlitePool) -> Result<Vec<UserRow>, Error> {
    sqlx::query_as::<_, UserRow>("SELECT user_id, identity_id FROM users")
        .fetch_all(pool)
        .await
}
#[tracing::instrument(err)]
pub async fn read_user(pool: &SqlitePool, user_id: String) -> Result<UserRow, Error> {
    sqlx::query_as::<_, UserRow>("SELECT user_id, identity_id FROM users WHERE user_id = ?")
        .bind(user_id)
        .fetch_one(pool)
        .await
}
#[tracing::instrument(err)]
pub async fn read_user_by_identity_id(
    pool: &SqlitePool,
    identity_id: String,
) -> Result<UserRow, Error> {
    sqlx::query_as::<_, UserRow>("SELECT user_id, identity_id FROM users WHERE identity_id = ?")
        .bind(identity_id)
        .fetch_one(pool)
        .await
}
#[tracing::instrument(err)]
pub async fn list_posts(pool: &SqlitePool) -> Result<Vec<PostRow>, Error> {
    sqlx::query_as::<_, PostRow>("SELECT post_id, user_id, content FROM posts")
        .fetch_all(pool)
        .await
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, FromRow)]
pub struct PostRow {
    pub post_id: String,
    pub user_id: String,
    pub content: String,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, FromRow)]
pub struct UserRow {
    pub user_id: String,
    pub identity_id: String,
}
impl UserRow {
    #[tracing::instrument(err)]
    pub async fn from_identity(
        pool: &SqlitePool,
        identity: &Identity,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE identity_id=?")
            .bind(&identity.id)
            .fetch_one(pool)
            .await
    }
}
