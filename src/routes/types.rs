use serde::{Deserialize,Serialize};
#[derive(Serialize,Deserialize)]
pub struct Creatorder{
    pub price:u64,
    pub quantity:f64,
    pub user_id:String,
    pub market:String,
    pub side:Side
}
#[derive(Serialize,Deserialize)]
pub enum Side{
    BUY,
    SELL
}
#[derive(Serialize,Deserialize)]
pub struct DeleteOrder{
    pub userid:String,
    pub orderid:String
}
#[derive(Serialize,Deserialize)]
pub struct ResponseCreateOrder{
    pub orderid:String,
    pub filled_qty:String,
    pub reaminag_qty:String,
}
pub struct ResponseDeleteOrder{
    pub response:bool,
    pub filled_qty:String,
    pub reaminag_qty:String
}