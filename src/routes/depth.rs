use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use crate::{redis::redismanager::RedisManager, types::to::{Markets, ToOrderbook}};



#[get("/depth")]
async  fn getdepth(query:web::Data<Markets>)->impl Responder{
 let redis=RedisManager::instance().lock().expect("Redis cant connect");
 

            let message=ToOrderbook::Depth(query.get_ref().clone());
            match redis.redis(message.clone()){
               Ok(response)=>HttpResponse::Ok().json(response),
               Err(err)=>HttpResponse::InternalServerError().body("Failed to get the depth")
            } 
        }
    
