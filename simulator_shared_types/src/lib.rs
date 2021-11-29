use bytemuck::{Pod, Zeroable};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Hash)]
#[repr(C)]
pub struct FileRecord<T> {
    pub label: T,
    pub size : i64
}

unsafe impl Zeroable for FileRecord<i32> {}

impl Copy for FileRecord<i32> {}

unsafe impl Pod for FileRecord<i32> {}