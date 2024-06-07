mod backend;

use actix_files::NamedFile;
use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use actix_web::guard;
use actix_web::{
    cookie::Key, get, put, web, App, HttpRequest, HttpResponse, HttpResponseBuilder, HttpServer,
    Responder,
};
use backend::db;
use std::env;
use std::path::PathBuf;

async fn index(req: HttpRequest) -> actix_web::Result<NamedFile> {
    let file: PathBuf = req.match_info().query("filename").parse().unwrap();
    let mut saved = env::current_dir()?
        .join("web")
        .join("dist")
        .join("assets")
        .join(file);
    let opened = NamedFile::open(&saved);
    if let Ok(file) = opened {
        Ok(file)
    } else {
        saved.set_file_name("error404.html");
        Ok(NamedFile::open(saved)?)
    }
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // backend::email::send_a_mail().await;
    //
    if !env::current_dir()?.join("static").is_dir() {
        panic!("Failed to load static dir")
    }
    db::surreal::init().await.expect("Failed to init db");

    let key = Key::generate();
    HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), key.clone())
                    .cookie_secure(false)
                    .build(),
            )
            .default_service(web::to(|| async { NamedFile::open("web/dist/index.html") }))
            .service(backend::web::robots)
            .service(backend::web::register_put)
            .service(backend::web::login_check)
            .service(web::scope("/verify").service(backend::web::verify::gen))
            .route("/nude", web::get().to(manual_hello))
            .route("/assets/{filename:.*}", web::get().to(index))
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
