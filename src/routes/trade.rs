use actix_web::{get, web, HttpResponse, Responder};

use crate::{redis::redismanager::RedisManager, types::to::{Markets, ToOrderbook}};


#[get("/trade")]
async fn gettrade(query:web::Data<Markets>)->impl Responder{
   let redis=RedisManager::instance().lock().expect("Unable to connect to redis");
   let message=query.get_ref().market.clone();
   match redis.redis(ToOrderbook::Depth(Markets { market: message })){
       Ok(respons)=>HttpResponse::Ok().json(respons),
       Err(err)=>HttpResponse::InternalServerError().body("Failed to get the trade")
   }
}