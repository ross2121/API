use actix_web::{post, web, HttpResponse, Responder};

use crate::{redis::redismanager::RedisManager, types::to::{CreateOrder,ToOrderbook}};


#[post("/create/order")]
async  fn createorder(body:web::Json<CreateOrder>)->impl Responder{
    let redis=RedisManager::instance().lock().unwrap();
    let body=body.into_inner();
    let message=ToOrderbook::CreateOrder(body);
    match redis.redis(message){
        Ok(response) => HttpResponse::Ok().json(response),
        Err(_) => {
 
            HttpResponse::InternalServerError().body("Failed to create order")
        }
    }

}
