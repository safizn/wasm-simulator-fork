use once_cell::sync::Lazy;
use std::sync::Mutex;
use algorithm::CacheAlgorithm;
use gdsize::GdSize;
use simulator_shared_types::FileRecord;


static POLICY : Lazy<Mutex<Option<GdSize<i32>>>> = Lazy::new(||{
    Mutex::new(None)
});

#[no_mangle]
pub fn init(size: i64){
    *POLICY.lock().unwrap() = Some(GdSize::<i32>::new(size));
}

static mut BUFFERS : Vec<Box<[u8]>> = Vec::new();

#[no_mangle]
pub fn alloc(size: i32) -> i64 {
    let buffer = Vec::<u8>::with_capacity(size as usize).into_boxed_slice();
    let ptr = buffer.as_ptr() as i32;
    unsafe{BUFFERS.push(buffer)};
    packed_i32::join_i32_to_i64(ptr, size )
}


#[no_mangle]
pub fn send(ptr: i32, buffer_size: i32){

    let slice = unsafe {
        // ptr as *const _ casts the i32 ptr to an actual pointer
        std::slice::from_raw_parts(ptr as *const _, buffer_size as usize)
        // from_raw_parts turns this data into a byte array we can use safely.
    };
    // Pop the buffer, but keep the memory allocated by assigning it to a variable.
    let _buffer_would_be_dropped  = unsafe {BUFFERS.pop()};

    // deserialize the slice.
    let x : FileRecord<i32> = bincode::deserialize(slice).expect("Deserialization error");

    POLICY.lock().unwrap().as_mut().unwrap().simulate(x);
}

#[no_mangle]
pub fn stats() -> i64 {
    let stats = POLICY.lock().unwrap().as_ref().unwrap().stats();
    packed_i32::join_i32_to_i64(stats.0, stats.1)
}