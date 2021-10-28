use std::hash::Hash;
use fifo::FiFo;
use simulator_shared_types::FileRecord;
use crate::policy::Policy;

pub struct NativeFiFo<T> {
    fifo: Option<FiFo<T>>
}

impl <T> NativeFiFo<T> {
    pub fn new() -> Self {
        NativeFiFo{
            fifo : None
        }
    }
}

impl <T> Policy<T> for NativeFiFo<T> where T : Hash + Eq + Clone{
    fn initialize(&mut self, cache_size: i64) {
       self.fifo = Some(FiFo::new(cache_size))
    }

    fn send_request(&mut self, pair: FileRecord<T>) {
        self.fifo.as_mut().unwrap().simulate(pair);
    }

    fn stats(&self) -> (i32, i32) {
        self.fifo.as_ref().unwrap().stats()
    }


}