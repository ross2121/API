use actix_web::{delete, post, web, HttpResponse, Responder};

use crate::{redis::redismanager::RedisManager, types::to::{CreateOrder, OrderCancel, ToOrderbook}};


#[post("/create/order")]
async  fn createorder(body:web::Json<CreateOrder>)->impl Responder{
    println!("check1");
    let redis=RedisManager::instance().lock().unwrap();
    let body=body.into_inner();
    let message=ToOrderbook::CreateOrder(body);
    println!("Check2");
    match redis.redis(message){
        Ok(response) => HttpResponse::Ok().json(response),
        Err(_) => {
 
            HttpResponse::InternalServerError().body("Failed to create order")
        }
    }

}
#[delete("/delete/order")]
async  fn deleteorder(body:web::Json<OrderCancel>)->impl Responder{
      let redis=RedisManager::instance().lock().unwrap();
      let body=body.into_inner();
      let message=ToOrderbook::OrderCancel(body);
      match  redis.redis(message) {
          Ok(response)=>HttpResponse::Ok().json(response),
          Err(_)=>{
            HttpResponse::InternalServerError().body("Failed to delete order")
          }
      }
      
}