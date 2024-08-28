/*
Copyright (c) 2024 Gravity3 by bluespada <pentingmain@gmail.com>
Copyright (c) 2024 All Gravity3 Contributor and Maintainer. 

This source code or software is under BSD 3 License please read
Accompanying file COPY or read online at https://opensource.org/license/bsd-3-clause.
*/
mod app;
mod utils;

use dotenv::dotenv;
use actix_web::{ App, HttpServer };

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // make sure dotenv is loaded.
    dotenv().ok();
    
    // start http server
    HttpServer::new(|| {
        println!("starting");
        App::new()
            //.default_service(web::get().to(default_handler))      
    })
    .bind(("127.0.0.1", 3000))?
    .workers(1)
    .run()
    .await
}