
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct CacheEvent<T : Serialize + Deserialize> {
    label: T,
    size : i32
}