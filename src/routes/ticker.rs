use actix_web::{get, web, HttpResponse, Responder};

use crate::{redis::redismanager::RedisManager, types::to::Markets};



#[get("/ticker")]
async fn getticker(query:web::Data<Markets>)->impl Responder{
   let   redis=RedisManager::instance().lock().expect("Not able to connect to redis");
   let message=query.get_ref().market.clone();
   match redis.redis(crate::types::to::ToOrderbook::Ticker(Markets { market: message })) {
        Ok(response)=>HttpResponse::Ok().json(response),
        Err(_)=>HttpResponse::InternalServerError().body("Error geting the ticker")
   } 
}