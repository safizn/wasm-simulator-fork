use once_cell::sync::Lazy;
use std::sync::Mutex;
use algorithm::CacheAlgorithm;
use lru::LRU;

use simulator_shared_types::FileRecord;


static POLICY : Lazy<Mutex<Option<LRU<i32>>>> = Lazy::new(||{
    Mutex::new(None)
});

#[no_mangle]
pub fn init(size: i64){
    *POLICY.lock().unwrap() = Some(LRU::<i32>::new(size));
}

static mut BUFFER : [u8; std::mem::size_of::<FileRecord<i32>>()] = [0; std::mem::size_of::<FileRecord<i32>>()];

#[no_mangle]
pub fn alloc(size: i32) -> i64 {
    let ptr = unsafe {BUFFER.as_ptr() as i32};
    packed_i32::join_i32_to_i64(ptr, size )
}


#[no_mangle]
pub fn send(ptr: i32, buffer_size: i32){

    // Dont need the ptr/buffersize, data can only be in one spot
    let slice = unsafe {
        &BUFFER
    };

    let x = bytemuck::from_bytes::<FileRecord<i32>>(slice).clone();

    POLICY.lock().unwrap().as_mut().unwrap().simulate(x);
}

#[no_mangle]
pub fn stats() -> i64 {
    let stats = POLICY.lock().unwrap().as_ref().unwrap().stats();
    packed_i32::join_i32_to_i64(stats.0, stats.1)
}