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



    // load method from module (basically get function pointer)

    let store = Store::default();
    println!("{:?}",std::env::current_dir());
    let path = Path::new("./modules/wasm32-unknown-unknown/debug/testModule.wasm");
    let module = Module::from_file(&store,path).expect("Module Not Found");

    // Prepare environment with imports
    let import_objects = imports!{};
    // Create new sandbox
    let instance = Instance::new(&module, &import_objects).expect("Failed to create instance");

    let multiply = instance.exports.get_function("multiply").expect("Failed to find method: multiply");


    let path = Path::new("./modules/wasm32-unknown-unknown/debug/loop_test_module.wasm");
    let module = Module::from_file(&store,path).expect("Module Not Found");

    // Prepare environment with imports
    let import_objects = imports!{};
    // Create new sandbox
    let instance = Instance::new(&module, &import_objects).expect("Failed to create instance");

    let multiply_many_times =instance.exports.get_function("multiply_many_times").expect("Failed to find method: multiply");



    let runs = 100;

    let native_duration = {
        let mut results = vec![];
        for _j in 1..runs{
            let start = std::time::Instant::now();
            for _i in 1..100_000 {
                let result = 2*3;

                assert_eq!(result,6);
            }
            let end = std::time::Instant::now();
            results.push((end-start).as_secs_f32())
        }
        results
    };

    let wasm_duration_hotload = {
        let mut results = vec![];
        for _j in 1..runs{
            let start = std::time::Instant::now();
            for _i in 1..100_000 {

                let result = multiply.call(&[Value::I32(2), Value::I32(3)]).expect("Failed to call method: multiply");

                assert_eq!(result[0], Value::I32(6));
            }
            let end = std::time::Instant::now();
            results.push((end-start).as_secs_f32())
        }
        results
    };

    let wasm_duration_preload = {
        let mut results = vec![];
        for _j in 1..runs{
            let start = std::time::Instant::now();
            let input = &[Value::I32(2), Value::I32(3)];
            for _i in 1..100_000 {

                let result = multiply.call(input).expect("Failed to call method: multiply");

                assert_eq!(result[0], Value::I32(6));
            }
            let end = std::time::Instant::now();
            results.push((end-start).as_secs_f32())
        }
        results
    };

    let native_average = (&native_duration.iter().fold(0.0,|acc, &num| acc + num))/ native_duration.len() as f32;
    let wasm_average_preload = (&wasm_duration_preload.iter().fold(0.0,|acc, &num| acc + num))/ wasm_duration_preload.len() as f32;
    let wasm_average_hotload = (&wasm_duration_hotload.iter().fold(0.0,|acc, &num| acc + num))/ wasm_duration_hotload.len() as f32;


    println!("Native: {:?} seconds \n Wasm Hotload: {:?} seconds \n Wasm Preload: {:?} seconds",
         native_duration,
         wasm_duration_hotload,
         wasm_duration_preload,
    );

    println!("Native Average: {:?} seconds \n Wasm Hotload Average: {:?} seconds \n Wasm Preload Average: {:?} seconds",
             native_average,
             wasm_average_hotload,
             wasm_average_preload,
    );

    let many_times = {
        let start = std::time::Instant::now();
        let result = multiply_many_times.call(&[Value::I32(2), Value::I32(3),Value::I32(100_000)]).expect("Failed to call method: multiply_many_times");
        let end = std::time::Instant::now();
        assert_eq!(result[0],Value::I32(6));
        end-start
    };

    println!("Multiply Many times: {}",many_times.as_secs_f32())

}
