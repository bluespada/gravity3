/*
Copyright (c) 2024 Gravity3 by bluespada <pentingmain@gmail.com>
Copyright (c) 2024 All Gravity3 Contributor and Maintainer. 

This source code or software is under BSD 3 License please read
Accompanying file COPY or read online at https://opensource.org/license/bsd-3-clause.
*/
mod app;
mod utils;

use dotenv::dotenv;
use std::env;
use actix_web::{ web, App, HttpResponse, HttpServer };

async fn default_handler() -> std::io::Result<HttpResponse> {

    Ok(
        HttpResponse::Ok().body("Hello from Yanto")
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // make sure dotenv is loaded.
    dotenv().ok();
    
    // setup from .env
    let app_host: String = env::var("host").unwrap_or("127.0.0.1".to_string());
    let app_port: u16 = env::var("port").unwrap_or("8000".to_string()).parse().unwrap();
    let app_workers: usize = env::var("workers").unwrap_or("4".to_string()).parse().unwrap();

    // start http server
    HttpServer::new(|| {
        println!("starting");
        App::new()
            .default_service(web::get().to(default_handler))      
    })
    .bind((app_host, app_port))?
    .workers(app_workers)
    .run()
    .await
}