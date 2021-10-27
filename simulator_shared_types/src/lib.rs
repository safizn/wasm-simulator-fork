
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Hash)]
pub struct FileRecord<T> {
    pub label: T,
    pub size : i32
}