use std::path::Path;
use bytemuck::__core::time::Duration;
use wasmer::{Instance, Module, Store, Value, imports};

pub fn pair_preload(store: &Store) -> Duration{
    let start = std::time::Instant::now();
    let path = Path::new("./modules/wasm32-unknown-unknown/release/testModule.wasm");
    let module = Module::from_file(&store,path).expect("Module Not Found");

    // Prepare environment with imports
    let import_objects = imports!{};
    // Create new sandbox
    let pair_instance = Instance::new(&module, &import_objects).expect("Failed to create instance");

    let input = &[Value::I32(2), Value::I32(3)];
    for _i in 0..100_000 {
        let multiply = pair_instance.exports.get_function("multiply").expect("Failed to find method: multiply");

        let result = multiply.call(input).expect("Failed to call method: multiply");

        assert_eq!(result[0], Value::I32(6));
    }
    let end = std::time::Instant::now();

    end - start
}

pub fn pair_preload_cached(store: &Store) -> Duration{
    let start = std::time::Instant::now();
    let path = Path::new("./modules/wasm32-unknown-unknown/release/testModule.wasm");
    let module = Module::from_file(&store,path).expect("Module Not Found");

    // Prepare environment with imports
    let import_objects = imports!{};
    // Create new sandbox
    let pair_instance = Instance::new(&module, &import_objects).expect("Failed to create instance");

    let multiply = pair_instance.exports.get_function("multiply").expect("Failed to find method: multiply");

    let input = &[Value::I32(2), Value::I32(3)];
    for _i in 0..100_000 {
        let result = multiply.call(input).expect("Failed to call method: multiply");

        assert_eq!(result[0], Value::I32(6));
    }
    let end = std::time::Instant::now();

    end - start
}

pub fn pair_hotload(store: &Store) -> Duration{
    let start = std::time::Instant::now();
    let path = Path::new("./modules/wasm32-unknown-unknown/release/testModule.wasm");
    let module = Module::from_file(&store,path).expect("Module Not Found");

    // Prepare environment with imports
    let import_objects = imports!{};
    // Create new sandbox
    let pair_instance = Instance::new(&module, &import_objects).expect("Failed to create instance");


    for _ in 0..100_000 {

        let multiply = pair_instance.exports.get_function("multiply").expect("Failed to find method: multiply");

        let result = multiply.call(&[Value::I32(2), Value::I32(3)]).expect("Failed to call method: multiply");

        assert_eq!(result[0], Value::I32(6));
    }
    let end = std::time::Instant::now();
    end - start
}

pub fn pair_hotload_cached(store: &Store) -> Duration{
    let start = std::time::Instant::now();
    let path = Path::new("./modules/wasm32-unknown-unknown/release/testModule.wasm");
    let module = Module::from_file(&store,path).expect("Module Not Found");

    // Prepare environment with imports
    let import_objects = imports!{};
    // Create new sandbox
    let pair_instance = Instance::new(&module, &import_objects).expect("Failed to create instance");

    let multiply = pair_instance.exports.get_function("multiply").expect("Failed to find method: multiply");

    for _ in 0..100_000 {

        let result = multiply.call(&[Value::I32(2), Value::I32(3)]).expect("Failed to call method: multiply");

        assert_eq!(result[0], Value::I32(6));
    }
    let end = std::time::Instant::now();
    end - start
}

pub fn multiply_many_test(store: &Store) -> Duration{
    let start = std::time::Instant::now();

    let path = Path::new("./modules/wasm32-unknown-unknown/release/loop_test_module.wasm");
    let module = Module::from_file(store,path).expect("Module Not Found");


    // Prepare environment with imports
    let import_objects = imports!{};
    // Create new sandbox
    let instance = Instance::new(&module, &import_objects).expect("Failed to create instance");

    let multiply_many_times =instance.exports.get_function("multiply_many_times").expect("Failed to find method: multiply");


    let result = multiply_many_times.call(&[Value::I32(2), Value::I32(3),Value::I32(100_000)]).expect("Failed to call method: multiply_many_times");
    let end = std::time::Instant::now();
    assert_eq!(result[0],Value::I32(6));
    end-start
}