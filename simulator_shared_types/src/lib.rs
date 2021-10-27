
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct CacheEvent<T>{
    pub label: T,
    pub size : i32
}