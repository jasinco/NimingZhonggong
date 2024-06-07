pub mod verify;

use crate::backend::db;
use actix_files::NamedFile;
use actix_session::Session;
use actix_web::{get, put, web, HttpRequest, HttpResponse, Responder, Result};

use super::db::error::RespondErr;

#[get("/robots.txt")]
pub async fn robots() -> impl Responder {
    NamedFile::open("static/robots.txt")
}

#[put("/login")]
pub async fn login_check(form: web::Json<db::User>, session: Session) -> Result<HttpResponse> {
    if let Ok(result) = db::surreal::query_user(form.0.name.as_str()).await {
        match form.verify_to(result) {
            Ok(()) => Ok(HttpResponse::Ok().finish()),
            Err(RespondErr::Failed) => Ok(HttpResponse::ExpectationFailed().finish()),
            Err(err) => Ok(HttpResponse::InternalServerError().body(err.to_string())),
        }
    } else {
        Ok(HttpResponse::TemporaryRedirect()
            .insert_header(("Location", "/register"))
            .body("No Account"))
    }
}

// Register jump to verify if the token is null, if not, add account
#[put("/register")]
pub async fn register_put(mut form: web::Json<db::User>, session: Session) -> Result<HttpResponse> {
    let verify = session.get::<bool>("verify")?;
    if let Some(verify_state) = verify {
        if verify_state {
            form.0.process_password();
            db::surreal::add_user(form.0);
            Ok(HttpResponse::Ok().body("verified"))
        } else {
            session.insert("formVal", form.0)?;
            Ok(HttpResponse::TemporaryRedirect()
                .append_header(("Location", "/verify"))
                .finish())
        }
    } else {
        session.insert("verify", false)?;
        session.insert("formVal", form.0)?;
        Ok(HttpResponse::TemporaryRedirect()
            .append_header(("Location", "/verify"))
            .finish())
    }
}
