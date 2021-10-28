
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Hash)]
pub struct FileRecord<T> {
    pub label: T,
    pub size : i64
}