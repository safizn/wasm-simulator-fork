#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use wasmer::Memory;

pub fn write_bincode_to_wasm_memory<T : serde::Serialize>(data: T, memory: &Memory, ptr: usize, len: usize){
    let serialized_array = bincode::serialize(&data).expect("Failed to serialize type");
    write_bytes_to_wasm_memory(&*serialized_array, memory, ptr, len)
}

pub fn write_bytemuck_to_wasm_memory<T : bytemuck::Pod >(data: T, memory: &Memory, ptr: usize, len: usize){
    let bytes = bytemuck::bytes_of(&data);
    //println!("Writing {} bytes using Bytemuck",bytes.len());
    write_bytes_to_wasm_memory(bytes, memory, ptr, len)
}

pub fn write_bytes_to_wasm_memory(bytes: &[u8], memory: &Memory, ptr: usize, len: usize){
    let mem_array: &mut [u8];
    unsafe {
        mem_array = memory.data_unchecked_mut();
        for i in 0..len {
            // iterate over the serialized struct, copying it to the memory of the target module,
            // using the ptr provided by caller
            mem_array[ptr + i as usize] = bytes[i as usize];
        }
    }
}