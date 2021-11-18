#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use benchmark_shared_data_structures::MultiplyParams;


static mut BUFFER : [u8; std::mem::size_of::<MultiplyParams>()] = [0; std::mem::size_of::<MultiplyParams>()];

#[no_mangle]
pub fn wasm_get_buffer() -> i64 {
    let ptr = unsafe {BUFFER.as_ptr() as i32};
    packed_i32::join_i32_to_i64(ptr, std::mem::size_of::<MultiplyParams>() as i32)
}

fn internal_struct_add(data : &MultiplyParams) -> i32{
    data.x * data.y
}

#[no_mangle]
pub fn struct_add(ptr: i32, buffer_size : i32) -> i32 {

    let slice = unsafe {
        &BUFFER
    };

    let x = bytemuck::from_bytes::<MultiplyParams>(slice);

    internal_struct_add(x)
}
