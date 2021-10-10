
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct MultiplyParams {
    pub x : i32,
    pub y : i32
}
