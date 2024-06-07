use once_cell::sync::OnceCell;
use surrealdb::engine::local::{Db, SpeeDb};

use surrealdb::Surreal;

use super::error::{DBResult, RespondErr};

static DB: OnceCell<Surreal<Db>> = OnceCell::new();

pub async fn init() -> DBResult<()> {
    let db = Surreal::new::<SpeeDb>("NimingDB").await?;
    db.use_db("users").await?;
    let _ = DB.set(db);
    Ok(())
}

pub async fn add_user(user_obj: super::User) -> DBResult<()> {
    let db = DB.get().unwrap();
    db.use_db("users").await?;

    let created: Option<super::User> = db
        .create(("user", user_obj.name.to_string()))
        .content(user_obj)
        .await?;
    if created.is_some() {
        Ok(())
    } else {
        Err(RespondErr::Failed)
    }
}
pub async fn query_user(username: &str) -> DBResult<super::User> {
    let db = DB.get().unwrap();
    db.use_db("users").await?;
    let selected: Vec<super::User> = db.select(username).await?;
    println!("{:#?}", selected);

    if let Some(user) = selected.into_iter().next() {
        Ok(user)
    } else {
        Err(RespondErr::Failed)
    }
}

pub async fn add_2fa_account(totp_obj: super::Totp2Fa) -> DBResult<()> {
    let db = DB.get().unwrap();
    db.use_db("2fa").await?;

    Ok(())
}
