use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
mod handlers;
mod model;

#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello_world api testing ")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get port from environment variable, default to 8080
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port = port.parse::<u16>().expect("PORT must be a number");

    // Bind to 0.0.0.0 to accept connections from any IP (required for Render)
    let bind_address = ("0.0.0.0", port);

    println!("Starting server at http://0.0.0.0:{}", port);

    HttpServer::new(|| {
        App::new()
            .route("/hey", web::get().to(manual_hello))
            .service(hello_world)
            .service(web::scope("/api").route("/items", web::post().to(handlers::create_item)))
            .route("/items", web::get().to(handlers::get_items))
            .route("/item/{id}", web::get().to(handlers::get_item))
            .route("/item/{id}", web::put().to(handlers::update_item))
            .route("/item/{id}", web::delete().to(handlers::delete_item))
    })
    .bind(bind_address)?
    .run()
    .await
}
