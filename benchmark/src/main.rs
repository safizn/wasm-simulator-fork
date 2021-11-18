use std::collections::HashMap;
use std::iter::Map;
use std::path::Path;
use std::time::Duration;
use wasmer::{Store, Module, Instance, imports, Value, Function, Memory};
use benchmark_shared_data_structures::MultiplyParams;

mod self_ref_test;
mod native_test;

struct Report {
    name : String,
    average : f64,
    standard_dev: f64,
}

fn main() {
    println!("Hello, world!");

    let mut benchmarks: HashMap<String, fn(&Store) -> Duration> = HashMap::<String, fn(&Store) -> Duration>::new();

    benchmarks.insert("Pair, Preload, Self-referential Struct".to_string(), self_ref_test::ouroboros_preload);
    benchmarks.insert("Pair, Hotload, Self-referential Struct".to_string(), self_ref_test::ouroboros_hotload);


    let runs = 100;

    let store = Store::default();

    let results : Vec<Report> = benchmarks.iter().map(|(name, func)|{

        let mut times : Vec<Duration> = vec![];

        for _ in 0..runs {
            let time = func(&store);
            times.push(time)
        }

        let average = (times.iter().fold(0.0,|acc, &num| acc + num.as_secs_f64()))/ runs as f64;

        Report{
            name: name.clone(),
            average,
            standard_dev: 0.0
        }
    }).collect();

    for r in results {
        println!("Benchmark: {:?} Average: {:?} Standard Dev: {:?}", r.name, r.average, r.standard_dev)
    }

    // load method from module (basically get function pointer)


    println!("{:?}",std::env::current_dir());
    let path = Path::new("./modules/wasm32-unknown-unknown/release/testModule.wasm");
    let module = Module::from_file(&store,path).expect("Module Not Found");

    // Prepare environment with imports
    let import_objects = imports!{};
    // Create new sandbox
    let pair_instance = Instance::new(&module, &import_objects).expect("Failed to create instance");

    let multiply = pair_instance.exports.get_function("multiply").expect("Failed to find method: multiply");

    let path = Path::new("./modules/wasm32-unknown-unknown/release/loop_test_module.wasm");
    let module = Module::from_file(&store,path).expect("Module Not Found");


    // Prepare environment with imports
    let import_objects = imports!{};
    // Create new sandbox
    let instance = Instance::new(&module, &import_objects).expect("Failed to create instance");

    let multiply_many_times =instance.exports.get_function("multiply_many_times").expect("Failed to find method: multiply");

    let path = Path::new("./modules/wasm32-unknown-unknown/release/bincode_addition.wasm");
    let module = Module::from_file(&store,path).expect("Module Not Found");

    // Prepare environment with imports
    let import_objects = imports!{};
    // Create new sandbox
    let struct_add_instance = Instance::new(&module, &import_objects).expect("Failed to create instance");

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

    let wasm_duration_hotload_cached_wasm = {
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

    let wasm_duration_hotload_cached_self = {
        let mut results = vec![];
        for _j in 1..runs{
            let dur = self_ref_test::ouroboros_hotload(&store);
            results.push((dur).as_secs_f32())
        }
        results
    };

    let wasm_duration_hotload = {
        let mut results = vec![];
        for _j in 1..runs{
            let start = std::time::Instant::now();
            for _i in 1..100_000 {
                let multiply = pair_instance.exports.get_function("multiply").expect("Failed to find method: multiply");

                let result = multiply.call(&[Value::I32(2), Value::I32(3)]).expect("Failed to call method: multiply");

                assert_eq!(result[0], Value::I32(6));
            }
            let end = std::time::Instant::now();
            results.push((end-start).as_secs_f32())
        }
        results
    };

    let wasm_duration_preload_cached_wasm = {
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

    let wasm_duration_preload_cached_self = {
        let mut results = vec![];
        for _j in 1..runs{
            let dur = self_ref_test::ouroboros_preload(&store);
            results.push(dur.as_secs_f32())
        }
        results
    };

    let wasm_duration_preload = {
        let mut results = vec![];
        for _j in 1..runs{
            let start = std::time::Instant::now();
            let input = &[Value::I32(2), Value::I32(3)];
            for _i in 1..100_000 {

                let multiply = pair_instance.exports.get_function("multiply").expect("Failed to find method: multiply");

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
    let wasm_average_preload_cached = (&wasm_duration_preload_cached_wasm.iter().fold(0.0,|acc, &num| acc + num))/ wasm_duration_preload_cached_wasm.len() as f32;
    let wasm_average_hotload_cached = (&wasm_duration_hotload_cached_wasm.iter().fold(0.0,|acc, &num| acc + num))/ wasm_duration_hotload_cached_wasm.len() as f32;
    let wasm_average_preload_cached_self = (&wasm_duration_preload_cached_self.iter().fold(0.0,|acc, &num| acc + num))/ wasm_duration_preload_cached_self.len() as f32;
    let wasm_average_hotload_cached_self = (&wasm_duration_hotload_cached_self.iter().fold(0.0,|acc, &num| acc + num))/ wasm_duration_hotload_cached_self.len() as f32;


    println!("Native: {:?} seconds \n Wasm Hotload: {:?} seconds \n Wasm Preload: {:?} seconds \n Wasm Hotload Cached: {:?} seconds \n Wasm Preload Cached: {:?} seconds \n Wasm Hotload Cached (Self-referential): {:?} seconds \n Wasm Preload Cached (Self-referential): {:?} seconds",
         native_duration,
         wasm_duration_hotload,
         wasm_duration_preload,
        wasm_duration_hotload_cached_wasm,
        wasm_duration_preload_cached_wasm,
        wasm_duration_hotload_cached_self,
        wasm_duration_preload_cached_self
    );

    println!("Native Average: {:?} seconds \n Wasm Hotload Average: {:?} seconds \n Wasm Preload Average: {:?} seconds \n Wasm Hotload Cached Average: {:?} \n Wasm Preload Cached Average: {:?} \n Wasm Hotload Cached (Self-referential) Average: {:?} seconds \n Wasm Preload Cached (Self-referential) Average: {:?} seconds",
             native_average,
             wasm_average_hotload,
             wasm_average_preload,
            wasm_average_hotload_cached,
            wasm_average_preload_cached,
            wasm_average_hotload_cached_self,
            wasm_average_preload_cached_self
    );

    let many_times = {
        let start = std::time::Instant::now();
        let result = multiply_many_times.call(&[Value::I32(2), Value::I32(3),Value::I32(100_000)]).expect("Failed to call method: multiply_many_times");
        let end = std::time::Instant::now();
        assert_eq!(result[0],Value::I32(6));
        end-start
    };

    println!("Multiply Many times: {}",many_times.as_secs_f32());

    let duration  = {
        let start = std::time::Instant::now();
        let params = MultiplyParams {
            x : 2,
            y : 3
        };
        for _ in 0..100_000{
            call_add_test(&params, &struct_add_instance)
        }
        let end = std::time::Instant::now();
        end-start
    };
    println!("Struct addition: {}",duration.as_secs_f32());

    let duration  = {
        let start = std::time::Instant::now();

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
        end-start
    };
    println!("Struct addition (Cached WASM Reference): {}",duration.as_secs_f32());

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



