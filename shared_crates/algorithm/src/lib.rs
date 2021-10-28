use simulator_shared_types::FileRecord;

pub trait CacheAlgorithm<T>{
    fn simulate(&mut self, file: FileRecord<T>);
    fn new(size: i64) -> Self;
    fn stats(&self) -> (i32,i32);
}