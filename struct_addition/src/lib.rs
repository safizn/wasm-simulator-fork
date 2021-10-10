#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use benchmark_shared_data_structures::MultiplyParams;


static mut BUFFERS : Vec<Box<[u8]>> = Vec::new();

#[no_mangle]
pub fn wasm_prepare_buffer(size: i32) -> i64 {
    let buffer = Vec::<u8>::with_capacity(size as usize).into_boxed_slice();
    let ptr = buffer.as_ptr() as i32;
    unsafe{BUFFERS.push(buffer)};
    join_i32_to_i64(ptr, size )
}

fn internal_struct_add(data : MultiplyParams) -> i32{
    data.x * data.y
}

#[no_mangle]
pub fn struct_add(ptr: i32, buffer_size : i32) -> i32 {

    let slice = unsafe {
        // ptr as *const _ casts the i32 ptr to an actual pointer
        std::slice::from_raw_parts(ptr as *const _, buffer_size as usize)
        // from_raw_parts turns this data into a byte array we can use safely.
    };
    // Pop the buffer, but keep the memory allocated by assigning it to a variable.
    let _buffer_would_be_dropped  = unsafe {BUFFERS.pop()};

    // deserialize the slice.
    let x = bincode::deserialize(slice).expect("Deserialization error");

    internal_struct_add(x)
}


/// Combine two i32 into one i64
fn join_i32_to_i64( a:i32, b:i32)->i64 {
    //((a as i64) << 32) | (b as i64)
    (((a as u64) << 32) | (b as u64) & 0x00000000ffffffff) as i64
}