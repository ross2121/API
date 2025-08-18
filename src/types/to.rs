

use serde::{Serialize,Deserialize};
use serde_json::{ Number};
#[derive(Serialize,Deserialize,Clone,PartialEq,Debug)]
pub enum Side  {
    Sell,Buy
}
#[derive(Serialize,Deserialize,Debug,Clone,PartialEq)]
#[serde(rename_all="camelCase")]
pub struct CreateOrder{
pub price :String,
pub qty:f64,
pub market:String,
pub side:Side,
pub user_id:String    
}
#[derive(Serialize,Deserialize,Clone,Debug,PartialEq)]
#[serde(rename_all="camelCase")]
pub struct Order{
  pub userid:String,
  pub market:String
}
#[derive(Serialize,Deserialize,Clone,Debug,PartialEq)]
#[serde(rename_all="camelCase")]
pub struct  Markets{
    pub  market:String
}

#[derive(Serialize,Deserialize,Clone,Debug,PartialEq)]
#[serde(rename_all="camelCase")]
pub struct OrderCancel{
pub oderid:String,
pub market:String,
}

#[derive(Serialize,Deserialize,Debug,Clone,PartialEq)]
#[serde(tag="type",content="payload")]
pub enum  ToOrderbook {
   #[serde(rename="CREATE_ORDER")]
    CreateOrder(CreateOrder),
    #[serde(rename="CANCEL_ORDER")]
    OrderPlaced(Order),
    #[serde(rename="GET_DEPTH")]
     OrderCancel(OrderCancel),
     #[serde(rename="GET_TRADE")]
     OpenOrder(Markets),
     #[serde(rename="GET_TICKER")]
     Trade( Markets),
     #[serde(rename="GET_OPEN_ORDER")]
     Ticker(Order)
}
