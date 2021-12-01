use std::path::Path;
use std::time::Duration;
use wasmer::{Instance, Module, Store, Value, imports, Function};
use benchmark_shared_data_structures::MultiplyParams;

pub fn bytemuck_test(store : &Store) -> Duration{
    let start = std::time::Instant::now();
    let path = Path::new("./modules/wasm32-unknown-unknown/release/bytemuck_addition.wasm");
    let module = Module::from_file(&store,path).expect("Module Not Found");

    // Prepare environment with imports
    let import_objects = imports!{};
    // Create new sandbox
    let bytemuck_add_instance = Instance::new(&module, &import_objects).expect("Failed to create instance");

    let params = MultiplyParams {
        x : 2,
        y : 3
    };
    for _ in 0..100_000{

        let prepare_buffer_fuc = bytemuck_add_instance
            .exports
            .get_function("wasm_prepare_buffer")
            .expect("No such function");

        let struct_add = bytemuck_add_instance
            .exports
            .get_function("struct_add")
            .expect("Could not find function struct_add");

        call_add_test_muck(params, &bytemuck_add_instance, prepare_buffer_fuc, struct_add)
    }
    let end = std::time::Instant::now();
    end-start
}

pub fn bytemuck_cached_test(store : &Store) -> Duration{
    let start = std::time::Instant::now();
    let path = Path::new("./modules/wasm32-unknown-unknown/release/bytemuck_addition.wasm");
    let module = Module::from_file(&store,path).expect("Module Not Found");

    // Prepare environment with imports
    let import_objects = imports!{};
    // Create new sandbox
    let bytemuck_add_instance = Instance::new(&module, &import_objects).expect("Failed to create instance");

    let params = MultiplyParams {
        x : 2,
        y : 3
    };

    let prepare_buffer_fuc = bytemuck_add_instance
        .exports
        .get_function("wasm_prepare_buffer")
        .expect("No such function");

    let struct_add = bytemuck_add_instance
        .exports
        .get_function("struct_add")
        .expect("Could not find function struct_add");
    for _ in 0..100_000{
        call_add_test_muck(params, &bytemuck_add_instance, prepare_buffer_fuc, struct_add)
    }
    let end = std::time::Instant::now();
    end-start
}

pub fn bytemuck_fixed_test(store : &Store) -> Duration{
    let start = std::time::Instant::now();
    let path = Path::new("./modules/wasm32-unknown-unknown/release/bytemuck_addition_fixed.wasm");
    let module = Module::from_file(&store,path).expect("Module Not Found");

    // Prepare environment with imports
    let import_objects = imports!{};
    // Create new sandbox
    let bytemuck_add_instance_fixed = Instance::new(&module, &import_objects).expect("Failed to create instance");
    let (ptr, len) =  packed_i32::split_i64_to_i32(bytemuck_add_instance_fixed.exports.get_function("wasm_get_buffer").unwrap().call(&[]).unwrap()[0].i64().unwrap());

    let params = MultiplyParams {
        x : 2,
        y : 3
    };
    for _ in 0..100_000{
        call_add_test_muck_fixed(params, &bytemuck_add_instance_fixed, ptr as usize, len as usize)
    }
    let end = std::time::Instant::now();
    end-start
}

pub fn bytemuck_cached_fixed_test(store: &Store) -> Duration{
    let start = std::time::Instant::now();
    let path = Path::new("./modules/wasm32-unknown-unknown/release/bytemuck_addition_fixed.wasm");
    let module = Module::from_file(&store,path).expect("Module Not Found");

    // Prepare environment with imports
    let import_objects = imports!{};
    // Create new sandbox
    let bytemuck_add_instance_fixed = Instance::new(&module, &import_objects).expect("Failed to create instance");
    let (ptr, len) =  packed_i32::split_i64_to_i32(bytemuck_add_instance_fixed.exports.get_function("wasm_get_buffer").unwrap().call(&[]).unwrap()[0].i64().unwrap());

    let struct_add = bytemuck_add_instance_fixed
        .exports
        .get_function("struct_add")
        .expect("Could not find function struct_add");

    let mem = bytemuck_add_instance_fixed.exports.get_memory("memory").expect("Could not get memory");

    let mparams = MultiplyParams {
        x : 2,
        y : 3
    };
    for _ in 0..100_000{


        let expected = mparams.x * mparams.y;

        runtime::write_bytemuck_to_wasm_memory(mparams, mem, ptr as usize, len as usize);

        // Now, call the method
        let result = struct_add.call(&[Value::I32(ptr),Value::I32(len)])
            .expect("Function call failed");

        assert_eq!(result[0].i32().expect("Was not i32"), expected);
    }
    let end = std::time::Instant::now();
    end-start
}




fn call_add_test_muck(params : MultiplyParams, instance : &Instance, get_mem: &Function, struct_add: &Function ){
    let buffer_size = std::mem::size_of::<MultiplyParams>();

    //println!("BUFFER_SIZE: {}", buffer_size);

    let result = get_mem.call(&[Value::I32(buffer_size as i32)]).expect("Function call failed");

    let compressed_nums = result[0].i64().expect("Was not i64");

    let (ptr, len) = packed_i32::split_i64_to_i32(compressed_nums);

    let mem = instance.exports.get_memory("memory").expect("Could not get memory");

    let expected = params.x * params.y;

    runtime::write_bytemuck_to_wasm_memory(params, mem, ptr as usize, len as usize);



    let result = struct_add.call(&[Value::I32(ptr),Value::I32(len)])
        .expect("Function call failed");

    assert_eq!(result[0].i32().expect("Was not i32"), expected);

}

fn call_add_test_muck_fixed(params : MultiplyParams, instance : &Instance, ptr : usize, len: usize){
    let mem = instance.exports.get_memory("memory").expect("Could not get memory");

    let expected = params.x * params.y;

    runtime::write_bytemuck_to_wasm_memory(params, mem, ptr, len);

    // Now, call the method
    let struct_add = instance
        .exports
        .get_function("struct_add")
        .expect("Could not find function struct_add");

    let result = struct_add.call(&[Value::I32(ptr as i32),Value::I32(len as i32)])
        .expect("Function call failed");

    assert_eq!(result[0].i32().expect("Was not i32"), expected);

}