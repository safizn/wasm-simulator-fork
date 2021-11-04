use std::hash::Hash;
use std::marker::PhantomData;
use algorithm::CacheAlgorithm;
use simulator_shared_types::FileRecord;
use crate::policy::PolicyModule;

pub struct NativePolicyModule<Alg,T> where Alg : CacheAlgorithm<T> {
    fifo: Option<Alg>,
    phantom: PhantomData<T>
}

impl <Alg,T> NativePolicyModule<Alg,T> where Alg : CacheAlgorithm<T> {
    pub fn new() -> Self {
        NativePolicyModule{
            fifo : None,
            phantom: PhantomData
        }
    }
}

impl <Alg,T> PolicyModule<T> for NativePolicyModule<Alg,T> where T : Hash + Eq + Clone, Alg: CacheAlgorithm<T>{
    fn initialize(&mut self, cache_size: i64) {
       self.fifo = Some(Alg::new(cache_size))
    }

    fn send_request(&mut self, pair: FileRecord<T>) {
        self.fifo.as_mut().unwrap().simulate(pair);
    }

    fn stats(&self) -> (i32, i32) {
        self.fifo.as_ref().unwrap().stats()
    }


}