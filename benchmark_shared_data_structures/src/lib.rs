use bytemuck::{Pod, Zeroable};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
#[repr(C)]
pub struct MultiplyParams {
    pub x : i32,
    pub y : i32
}
// Marker traits for bytemuck
unsafe impl Zeroable for MultiplyParams {}
unsafe impl Pod for MultiplyParams {}

