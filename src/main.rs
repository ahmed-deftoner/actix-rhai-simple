use actix_web::{
    HttpServer,
    App,
    get,
    Responder,
    web::Path
};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(multiply)
            .service(add)
            .service(subtract)
            .service(divide)
    })
        .bind(("127.0.0.1", 8080))
        .unwrap()
        .run()
        .await
}
