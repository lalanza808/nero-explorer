extern crate tokio;
extern crate actix_web;
extern crate awc;

use actix_web::{get, web, App, HttpServer, Responder};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    let client = awc::Client::default();
    let req = client.get("http://node.suchwow.xyz:34568/get_info");
    let mut res = req.send().await.unwrap();
    format!("Hello {name}! Response: {:?}", res.body().await.unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let env_url = env::var("DAEMON_URI");
    HttpServer::new(|| {
        App::new().service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
