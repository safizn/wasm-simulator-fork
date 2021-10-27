use std::hash::Hash;
use fifo::FiFo;
use simulator_shared_types::FileRecord;
use crate::policy::Policy;

pub struct NativeFiFo<T> {
    fifo: Option<FiFo<T>>
}

impl <T> Policy<T> for NativeFiFo<T> where T : Hash + Eq {
    fn initialize(&mut self, cache_size: i32) {
       self.fifo = Some(FiFo::new(cache_size))
    }

    fn send_request(&mut self, pair: FileRecord<T>) {
        &self.fifo.as_mut().unwrap().simulate(pair);
    }

    fn state(&self) -> (i32, i32) {
        self.fifo.as_ref().unwrap().stats()
    }
}