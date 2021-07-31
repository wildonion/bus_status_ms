







// -------------------------------------------------------------------------------------
// NOTE - on VPS, before build : sudo chmod 777 NAME_OF_PROJECT_FOLDER/
// NOTE - cargo install cargo-watch
// build it => cargo build --bin reports --release
// watch it => cargo watch -x run
// EXAMPLE - http://localhost:7366/avl/api/reports/status/166/2021-04-12T00:00:00+06:00/2021-05-12T23:59:59+06:00
// -------------------------------------------------------------------------------------




mod entities;
mod handlers;
use actix_web::{App, web};
use actix_cors::Cors;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use actix_web::Responder;
use listenfd::ListenFd;
use actix_web::middleware::Logger;
use std::env;
use dotenv::dotenv;
use self::entities::bus_status::api::init_service;






async fn index() -> impl Responder {
    HttpResponse::Ok().body("<h1>WELCOME 2 AVL REPORTS API</h1>")
}





#[actix_web::main]
async fn main() -> std::io::Result<()> { //-- return type is an empty Result object - std::io::Result is broadly used across std::io for any operation which may produce an error.
    




        env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
        env_logger::init();
        dotenv().expect("⚠️ .env file not found");
        let secret_key = env::var("JWT_SECRET_KEY").expect("⚠️ no jwt secret key set"); // TODO - 
        let environment = env::var("ENVIRONMENT").expect("⚠️ no environment variable set"); // TODO - 
        let compony = env::var("COMPONY_NAME").expect("⚠️ no compony name set"); // TODO - 
        



        let mut listenfd = ListenFd::from_env();
        let mut server = 
            HttpServer::new(|| { //-- building and returning the app inside the HttpServer::new closure
                App::new()
                    .configure(init_service)
                    .route("/avl/api/reports/status", web::get().to(index)) //-- default route
                    .wrap(Logger::default())
            });
        
        

        
        server = match listenfd.take_tcp_listener(0)?{
            Some(listener) => server.listen(listener)?,
            None => {
                let host = env::var("HOST").expect("⚠️ please set host in .env");
                let port = env::var("PORT").expect("⚠️ please set port in .env");
                server.bind(format!("{}:{}", host, port))?
            }
        };

        

        server.run().await

}
