mod types;
mod redis;
mod routes;


use actix_web::{web, App, HttpServer};

use crate::routes::depth::getdepth;
use crate::routes::order::{createorder, deleteorder,getaccount};
use crate::routes::ticker::getticker;
use crate::routes::trade::gettrade;
    #[actix_web::main]
async  fn main()->std::io::Result<()>{
    println!("hello");
   HttpServer::new(||
    App::new().service(createorder).service(deleteorder).service(getaccount).service(getdepth).service(gettrade).service(getticker)).bind(("127.0.0.1",8080))?.run().await
}

