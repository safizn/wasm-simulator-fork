use std::fs::File;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use lfu::LFU;
use simulator_shared_types::FileRecord;
use algorithm::CacheAlgorithm;


static POLICY : Lazy<Mutex<Option<LFU<i32>>>> = Lazy::new(||{
    Mutex::new(None)
});

#[no_mangle]
pub fn init(size: i64){
    *POLICY.lock().unwrap() = Some(LFU::<i32>::new(size));
}
#[no_mangle]
pub fn send(label: i32, size : i64){
    POLICY.lock().unwrap().as_mut().unwrap().simulate(FileRecord::<i32>{
        label,
        size
    })
}

#[no_mangle]
pub fn stats() -> i64 {
    let stats = POLICY.lock().unwrap().as_ref().unwrap().stats();
    packed_i32::join_i32_to_i64(stats.0, stats.1)
}
