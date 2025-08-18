mod types;
mod redis;
mod routes;

use actix_web::{web, App, HttpServer};

use crate::routes::main;
use crate::routes::order::{createorder, deleteorder};
use crate::types::from;
use crate::redis::redismanager;



    #[actix_web::main]
async  fn main()->std::io::Result<()>{
    println!("hello");
   HttpServer::new(||
    App::new().service(createorder).service(deleteorder)).bind(("127.0.0.1",8080))?.run().await
}

