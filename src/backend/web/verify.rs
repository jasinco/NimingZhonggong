use actix_session::Session;
use actix_web::{get, put, web, HttpMessage, HttpRequest, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};

use crate::backend::{db, email};

#[get("/gen")]
pub async fn gen(session: Session) -> Result<HttpResponse> {
    if let Some(form) = session.get::<db::User>("formVal")? {
        // let code = email::send_a_verify_mail(form.fetch_id()).await;
        // session.insert("code", code)?;
        Ok(HttpResponse::Created().finish())
    } else {
        Ok(HttpResponse::Ok().finish())
    }
}

#[derive(Debug, Deserialize)]
pub struct Code {
    code: String,
}

#[put("/")]
pub async fn verify(code: web::Json<Code>, session: Session) -> Result<HttpResponse> {
    if let Some(code1) = session.get::<String>("code")? {
        if code1 == code.0.code {
            Ok(HttpResponse::Ok().finish())
        } else {
            Ok(HttpResponse::NonAuthoritativeInformation().finish())
        }
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}
