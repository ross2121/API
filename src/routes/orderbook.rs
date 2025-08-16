use serde::{Serialize,Deserialize};
use crate::CreateOrder;

pub fn add_order(create_order: CreateOrder) -> Result<(), String> {

    println!("Adding order: {:?}", create_order);
    Ok(())
}
