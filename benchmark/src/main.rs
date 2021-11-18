
use std::path::Path;
use std::time::Duration;
use wasmer::{Store, Module, Instance, imports, Value};
use benchmark_shared_data_structures::MultiplyParams;

mod self_ref_test;
mod native_test;
mod pair_test;
mod bincode_test;

struct Report {
    name : String,
    average : f64,
    standard_dev: f64,
}

fn main() {

    let mut benchmarks: Vec<(String, fn(&Store) -> Duration)> = vec![];

    benchmarks.push(("Native Test".to_string(), native_test::native_test));
    benchmarks.push(("Pair, Preload".to_string(),pair_test::pair_preload));
    benchmarks.push(("Pair, Hotload".to_string(),pair_test::pair_hotload));
    benchmarks.push(("Pair, Preload, Cached".to_string(),pair_test::pair_preload_cached));
    benchmarks.push(("Pair, Hotload, Cached".to_string(),pair_test::pair_hotload_cached));
    benchmarks.push(("Pair, Preload, Self-referential Struct".to_string(), self_ref_test::ouroboros_preload));
    benchmarks.push(("Pair, Hotload, Self-referential Struct".to_string(), self_ref_test::ouroboros_hotload));
    benchmarks.push(("Pair, WASM Loop".to_string(), pair_test::multiply_many_test));
    benchmarks.push(("Bincode".to_string(),bincode_test::bincode_test));
    benchmarks.push(("Bincode, Cached".to_string(),bincode_test::bincode_cached));

    let runs = 1000;

    let store = Store::default();

    let results : Vec<Report> = benchmarks.iter().map(|(name, func)|{

        let mut times : Vec<Duration> = vec![];

        for _ in 0..runs {
            let time = func(&store);
            times.push(time)
        }

        let average: f64 = times.iter().map(|i| i.as_secs_f64() ).sum::<f64>()/ runs as f64;

        let standard_dev : f64 = (times.iter()
            .map(|i| i.as_secs_f64()-average)
            .map(|i| i.powf(2.0))
            .sum::<f64>())/ runs as f64;

        Report{
            name: name.clone(),
            average,
            standard_dev
        }
    }).collect();

    for r in results {
        println!("Benchmark: {:?} Average: {:?} Standard Dev: {:?}", r.name, r.average, r.standard_dev)
    }

    let path = Path::new("./modules/wasm32-unknown-unknown/release/bytemuck_addition.wasm");
    let module = Module::from_file(&store,path).expect("Module Not Found");

    // Prepare environment with imports
    let import_objects = imports!{};
    // Create new sandbox
    let bytemuck_add_instance = Instance::new(&module, &import_objects).expect("Failed to create instance");

    let path = Path::new("./modules/wasm32-unknown-unknown/release/bytemuck_addition_fixed.wasm");
    let module = Module::from_file(&store,path).expect("Module Not Found");

    // Prepare environment with imports
    let import_objects = imports!{};
    // Create new sandbox
    let bytemuck_add_instance_fixed = Instance::new(&module, &import_objects).expect("Failed to create instance");

    let duration  = {
        let start = std::time::Instant::now();
        let params = MultiplyParams {
            x : 2,
            y : 3
        };
        for _ in 0..100_000{
            call_add_test_muck(params, &bytemuck_add_instance)
        }
        let end = std::time::Instant::now();
        end-start
    };
    println!("Bytes Struct addition: {}",duration.as_secs_f32());



    let duration  = {
        let start = std::time::Instant::now();

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
    };
    println!("Bytes Struct addition (Fixed Memory): {}",duration.as_secs_f32());


    let duration  = {
        let start = std::time::Instant::now();

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
    };
    println!("Bytes Struct addition (Fixed Memory, Cached WASM reference): {}",duration.as_secs_f32());

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

fn call_add_test_muck(params : MultiplyParams, instance : &Instance ){
    let buffer_size = std::mem::size_of::<MultiplyParams>();

    //println!("BUFFER_SIZE: {}", buffer_size);
    let prepare_buffer_fuc = instance
        .exports
        .get_function("wasm_prepare_buffer")
        .expect("No such function");

    let result = prepare_buffer_fuc.call(&[Value::I32(buffer_size as i32)]).expect("Function call failed");

    let compressed_nums = result[0].i64().expect("Was not i64");

    let (ptr, len) = packed_i32::split_i64_to_i32(compressed_nums);

    let mem = instance.exports.get_memory("memory").expect("Could not get memory");

    let expected = params.x * params.y;

    runtime::write_bytemuck_to_wasm_memory(params, mem, ptr as usize, len as usize);

    // Now, call the method
    let struct_add = instance
        .exports
        .get_function("struct_add")
        .expect("Could not find function struct_add");

    let result = struct_add.call(&[Value::I32(ptr),Value::I32(len)])
        .expect("Function call failed");

    assert_eq!(result[0].i32().expect("Was not i32"), expected);

}
