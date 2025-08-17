use once_cell::sync::Lazy;
use std::sync::Mutex;
use redis::{Client, Commands, PubSubCommands, RedisResult};
use serde::{Serialize};

use crate::types::to::ToOrderbook;
use uuid::Uuid;
use crate::types::from::FromOrderbook;
#[derive(Serialize,Debug)]
pub struct Messagetype{
    userid:String,
    message:ToOrderbook
}
pub struct RedisManager{
    client:Client,
    pubsub:Client
}
pub static  INSTANCE:Lazy<Mutex<RedisManager>>=Lazy::new(||{Mutex::new(RedisManager::main())});
impl RedisManager{
    pub fn instance()-> & 'static Mutex<RedisManager>{
      &INSTANCE
    }
  pub fn main()->Self{
    let redis_url = "redis://localhost:6379";
          let client=Client::open(redis_url).unwrap();
          let pubsub=Client::open(redis_url).unwrap();
          RedisManager { client:client, pubsub:pubsub }
    }
    pub fn redis(&self,message:ToOrderbook)->RedisResult<FromOrderbook>{
       let mut client=self.client.get_connection().unwrap();
       let mut pubsub=self.pubsub.get_connection().unwrap();
       let id = Uuid::new_v4();
      let mut pubsubcon=pubsub.as_pubsub();
       pubsubcon.subscribe(id.to_string()).expect("Error subscribing");
      let message=Messagetype{
        userid:id.to_string(),
        message:message
    };
    let json: () =client.lpush("message",serde_json::to_string(&message).expect("Not able to send message")).unwrap();
      let msg=pubsubcon.get_message().expect("Unable to get message");
      let  data:String=msg.get_payload().expect("No data");
      pubsubcon.unsubscribe(id.to_string()).unwrap();
      let response_data: FromOrderbook =serde_json::from_str(&data).expect("No response from orderbook");
      Ok(response_data)
    }
}
  