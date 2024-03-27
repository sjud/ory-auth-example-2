use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, FromRow};

// This will just map into ServerFnError when we call it in our serverfunctions with ? error handling
use sqlx::Error;
#[tracing::instrument(err)]
pub async fn create_user(
    pool: &SqlitePool,
    identity_id: &String,
    email: &String,
) -> Result<(), Error> {
    sqlx::query("INSERT INTO users (user_id,identity_id,email) VALUES (?,?,?)")
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(identity_id)
        .bind(email)
        .execute(pool)
        .await?;
    Ok(())
}

/// Returns the POST ROW
#[tracing::instrument(ret)]
pub async fn create_post(
    pool: &SqlitePool,
    user_id: &String,
    content: &String,
) -> Result<PostRow, Error> {
    sqlx::query_as::<_, PostRow>(
        "INSERT INTO posts (post_id,user_id,content) VALUES (?,?,?) RETURNING *",
    )
    .bind(uuid::Uuid::new_v4().to_string())
    .bind(user_id)
    .bind(content)
    .fetch_one(pool)
    .await
}
#[tracing::instrument]
pub async fn edit_post(pool: &SqlitePool, post_id: &String, content: &String) -> Result<(), Error> {
    sqlx::query("UPDATE posts SET content = ?  WHERE post_id = ?")
        .bind(post_id)
        .bind(content)
        .execute(pool)
        .await?;
    Ok(())
}
#[tracing::instrument]
pub async fn delete_post(pool: &SqlitePool, post_id: &String) -> Result<(), Error> {
    sqlx::query("DELETE FROM posts where post_id = ?")
        .bind(post_id)
        .execute(pool)
        .await?;
    Ok(())
}
#[tracing::instrument]
pub async fn list_users(pool: &SqlitePool) -> Result<Vec<UserRow>, Error> {
    sqlx::query_as::<_, UserRow>("SELECT user_id, identity_id FROM users")
        .fetch_all(pool)
        .await
}
#[tracing::instrument]
pub async fn read_user(pool: &SqlitePool, user_id: &String) -> Result<UserRow, Error> {
    sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE user_id = ?")
        .bind(user_id)
        .fetch_one(pool)
        .await
}
#[tracing::instrument(ret)]
pub async fn read_user_by_identity_id(
    pool: &SqlitePool,
    identity_id: &String,
) -> Result<UserRow, Error> {
    sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE identity_id = ?")
        .bind(identity_id)
        .fetch_one(pool)
        .await
}
#[tracing::instrument]
pub async fn read_user_by_email(pool: &SqlitePool, email: &String) -> Result<UserRow, Error> {
    sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE email = ?")
        .bind(email)
        .fetch_one(pool)
        .await
}
#[tracing::instrument]
pub async fn list_posts(pool: &SqlitePool) -> Result<Vec<PostRow>, Error> {
    sqlx::query_as::<_, PostRow>("SELECT post_id, user_id, content FROM posts")
        .fetch_all(pool)
        .await
}
#[tracing::instrument]
pub async fn create_post_permissions(pool:&SqlitePool, post_id:&String,user_id:&String,PostPermissions{read,write,edit,delete}:PostPermissions) -> Result<(),Error> {
    sqlx::query("INSERT INTO post_permissions (permission_id,post_id,post_id,read,write,edit,delete) VALUES (?,?,?,?,?,?,?)")
    .bind(uuid::Uuid::new_v4().to_string())
    .bind(post_id)
    .bind(user_id)
    .bind(read)
    .bind(write)
    .bind(edit)
    .bind(delete)
    .execute(pool)
    .await?;
    Ok(())
}

#[derive(Debug,PartialEq,Clone,Copy)]
pub struct PostPermission{
    pub read:bool,
    pub write:bool,
    pub edit:bool,
    pub delete:bool,
}

impl PostPermission{
    pub async fn from_db_call(pool:&SqlitePool,user_id:&String,post_id:&String) -> Result<Self,Error> {
        let row = sqlx::query_as::<_, PostPermissionRow>("SELECT * FROM post_permissions WHERE post_id = ? AND user_id = ?")
        .bind(post_id)
        .bind(user_id) 
        .fetch_one(pool)
        .await;
        Self::from(row)
    }
}

impl From<PostPermissionRow> for PostPermission{
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, FromRow)]
pub struct PostPermissionRow {
    pub permission_id:String,
    pub post_id: String,
    pub user_id: String,
    pub read:bool,
    pub write:bool,
    pub edit:bool,
    pub delete:bool,
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
    pub email: String,
}
