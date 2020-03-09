use actix_files::NamedFile;
use actix_web::{ web, App, HttpResponse, HttpServer, Responder, HttpRequest, Result};
use std::path::PathBuf;
use std::fs;
use std::env;

async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let req_file = req.match_info().query("filename");
    let relative_path = format!("{}{}", "../plots-bm-wasm/", req_file);
    println!("Relative Path {:?}", relative_path);
    let path: PathBuf = relative_path.parse().unwrap();
    println!("Absolute Path {:?}", fs::canonicalize(&relative_path));
    Ok(NamedFile::open(path)?)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Current Dir {:?}", env::current_dir());
    HttpServer::new(|| {
        App::new()
            .route("/{filename:.*}", web::get().to(index))
            .route("/again", web::get().to(index2))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
