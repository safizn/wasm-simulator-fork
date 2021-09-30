use std::path::Path;
use wasmer::{
    Store,
    Module,
    Instance,
    imports,
    Value
};

fn main() {
    println!("Hello, world!");
    let store = Store::default();
    println!("{:?}",std::env::current_dir());
    let path = Path::new("./modules/wasm32-unknown-unknown/debug/testModule.wasm");
    let module = Module::from_file(&store,path).expect("Module Not Found");

    // Prepare environment with imports
    let import_objects = imports!{};
    // Create new sandbox
    let instance = Instance::new(&module, &import_objects).expect("Failed to create instance");

    // load method from module (basically get function pointer)
    let multiply = instance.exports.get_function("multiply").expect("Failed to find method: multiply");

    let result = multiply.call(&[Value::I32(2),Value::I32(2)]).expect("Failed to call method: multiply");

    assert_eq!(result[0], Value::I32(4));

    let rust_duration = {
        let start = std::time::Instant::now();
        for i in 1..100_000_000 {
            let result = 2*2;

            assert_eq!(result,4);
        }
        let end = std::time::Instant::now();
        end-start
    };

    let wasm_duration_hotload = {
        let start = std::time::Instant::now();
        for i in 1..100_000_000 {

            let result = multiply.call(&[Value::I32(2), Value::I32(2)]).expect("Failed to call method: multiply");

            assert_eq!(result[0], Value::I32(4));
        }
        let end = std::time::Instant::now();
        end-start
    };

    let wasm_duration_preload = {
        let input = &[Value::I32(2), Value::I32(2)];
        let start = std::time::Instant::now();
        for i in 1..100_000_000 {

            let result = multiply.call(input).expect("Failed to call method: multiply");

            assert_eq!(result[0], Value::I32(4));
        }
        let end = std::time::Instant::now();
        end-start
    };

    println!("Native: {} seconds | Wasm Hotload: {} seconds | Wasm Preload: {} seconds",
        rust_duration.as_secs_f32(),
        wasm_duration_hotload.as_secs_f32(),
        wasm_duration_preload.as_secs_f32()
    )

}
