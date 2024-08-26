use actix_web::{ App, web, HttpServer, HttpRequest, HttpResponse };

async fn default_handler() -> std::io::Result<HttpResponse> {

    Ok(
        HttpResponse::Ok().body("Hello from Yanto")
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        println!("starting");
        App::new()
        .default_service(web::get().to(default_handler))      
    })
    .bind(("0.0.0.0", 3000))?
    .workers(1)
    .run()
    .await
}