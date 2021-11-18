use std::path::Path;
use bytemuck::__core::time::Duration;
use wasmer::{Function, Instance, Memory, Module, Store, Value, imports};
use benchmark_shared_data_structures::MultiplyParams;

pub fn bincode_test(store : &Store) -> Duration{
    let start = std::time::Instant::now();
    let path = Path::new("./modules/wasm32-unknown-unknown/release/bincode_addition.wasm");
    let module = Module::from_file(&store,path).expect("Module Not Found");

    // Prepare environment with imports
    let import_objects = imports!{};
    // Create new sandbox
    let struct_add_instance = Instance::new(&module, &import_objects).expect("Failed to create instance");
    let params = MultiplyParams {
        x : 2,
        y : 3
    };
    for _ in 0..100_000{
        call_add_test(&params, &struct_add_instance)
    }
    let end = std::time::Instant::now();
    end-start
}


pub fn bincode_cached(store: &Store) -> Duration{

    let start = std::time::Instant::now();
    let path = Path::new("./modules/wasm32-unknown-unknown/release/bincode_addition.wasm");
    let module = Module::from_file(&store,path).expect("Module Not Found");

    // Prepare environment with imports
    let import_objects = imports!{};
    // Create new sandbox
    let struct_add_instance = Instance::new(&module, &import_objects).expect("Failed to create instance");

    let mem = struct_add_instance.exports.get_memory("memory").expect("Could not get memory");

    let struct_add = struct_add_instance
        .exports
        .get_function("struct_add")
        .expect("Could not find function struct_add");

    let prepare_buffer_fuc = struct_add_instance
        .exports
        .get_function("wasm_prepare_buffer")
        .expect("No such function");

    let params = MultiplyParams {
        x : 2,
        y : 3
    };
    for _ in 0..100_000{
        call_add_test_cached(&params, struct_add,prepare_buffer_fuc,mem)
    }
    let end = std::time::Instant::now();
    end - start
}

fn call_add_test_cached(params : &MultiplyParams, struct_add : &Function, prepare_buffer_fuc: &Function, mem: &Memory){
    let buffer_size = bincode::serialized_size(params).expect("Could not calculate buffer size") as i32;

    let result = prepare_buffer_fuc.call(&[Value::I32(buffer_size)]).expect("Function call failed");

    let compressed_nums = result[0].i64().expect("Was not i64");

    let (ptr, len) = packed_i32::split_i64_to_i32(compressed_nums);

    // let mem = instance.exports.get_memory("memory").expect("Could not get memory");

    runtime::write_bincode_to_wasm_memory(params, mem, ptr as usize, len as usize);

    // // Now, call the method
    // let struct_add = instance
    //     .exports
    //     .get_function("struct_add")
    //     .expect("Could not find function struct_add");

    let result = struct_add.call(&[Value::I32(ptr),Value::I32(len)])
        .expect("Function call failed");

    assert_eq!(result[0].i32().expect("Was not i32"), params.x * params.y);

}

fn call_add_test(params : &MultiplyParams, instance : &Instance){
    let buffer_size = bincode::serialized_size(params).expect("Could not calculate buffer size") as i32;

    let prepare_buffer_fuc = instance
        .exports
        .get_function("wasm_prepare_buffer")
        .expect("No such function");

    let result = prepare_buffer_fuc.call(&[Value::I32(buffer_size)]).expect("Function call failed");

    let compressed_nums = result[0].i64().expect("Was not i64");

    let (ptr, len) = packed_i32::split_i64_to_i32(compressed_nums);

    let mem = instance.exports.get_memory("memory").expect("Could not get memory");

    runtime::write_bincode_to_wasm_memory(params, mem, ptr as usize, len as usize);

    // Now, call the method
    let struct_add = instance
        .exports
        .get_function("struct_add")
        .expect("Could not find function struct_add");

    let result = struct_add.call(&[Value::I32(ptr),Value::I32(len)])
        .expect("Function call failed");

    assert_eq!(result[0].i32().expect("Was not i32"), params.x * params.y);

}
