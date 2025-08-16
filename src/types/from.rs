use std::clone;

use serde::{Serialize,Deserialize};
use serde_json::{ser, Number};
#[derive(Serialize,Deserialize,Clone,PartialEq)]
pub enum Side  {
    Sell,Buy
}
#[derive(Serialize,Deserialize,Debug,Clone,PartialEq)]
#[serde(rename_all="camelCase")]
pub struct Fill{
pub price :String,
pub qty:f64,
pub trade_id:f64    
}
#[derive(Serialize,Deserialize,Clone,PartialEq)]
#[serde(rename_all="camelCase")]
pub struct Trade{
  pub isbuyer:bool,
  pub price:String,
  pub quantity:String,
  pub symbol:String,
  pub market:String
}
#[derive(Serialize,Deserialize,Clone,PartialEq)]
#[serde(rename_all="camelCase")]
pub struct  Ticker{
    pub lastprice:Number,
    pub highestbid:Number,
   pub  lowestask:Number,
   pub volume24h:Number
}
#[derive(Serialize,Deserialize,Clone,PartialEq)]
#[serde(rename_all="camelCase")]
pub struct  Depth{
    pub market:String,
    pub bids:Vec<(String,String)>,
    pub ask:Vec<(String,String)>
    
}
#[derive(Serialize,Deserialize,Clone,PartialEq)]
#[serde(rename_all="camelCase")]
pub struct OrderCancel{
pub oderid:String,
pub executedqty:Number,
pub remainingqty:Number
}
#[derive(Serialize,Deserialize,Clone,PartialEq)]
#[serde(rename_all="camelCase")]
pub struct Orderstruct{
    pub orderid:String,
    pub executedid:String,
    pub price:String,
    pub userid:String,
    pub side:Side,
    pub quantity:String
}
#[derive(Serialize,Deserialize,PartialEq,Clone)]
#[serde(rename_all="camelCase")]
pub struct  Placeorder{
 pub orderid:String,
 pub executedqty:String,
 pub fills:Vec<Fill>
}

