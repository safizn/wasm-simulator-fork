use std::collections::{HashMap, HashSet};
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;
use clap::{App, Arg};
use wasmer::{Module, Store};
use fifo::FiFo;
use gdsize::GdSize;
use lfu::LFU;
use lru::LRU;
use simulator_shared_types::FileRecord;
use crate::native_modules::NativePolicyModule;
use crate::policy::{PolicyModule, WasmBincodePolicyModule, WasmPairPolicyModule};

mod policy;
mod native_modules;


fn main() {

    let matches = App::new("Caching Policy Simulator")
        .version("0.1")
        .author("Devon Hockley")
        .about("A caching policy simulator implemented using WASM")
        .arg(Arg::with_name("sample")
            .help("Sets the input data sample")
            .index(1)
            .required(true)
        )
        .get_matches();

    let file_path = matches.value_of("sample").unwrap();

    println!("Using input file: {}", file_path);



    let file = File::open(file_path).unwrap();
    let mut decompressed = GzDecoder::new(file);



    let mut string = String::new();
    decompressed.read_to_string(&mut string).unwrap();

    let data : Vec<FileRecord<i32>> = string.lines().map(
        |line| {
            if let [first, second, ..] = line.trim().split_ascii_whitespace().collect::<Vec<&str>>().as_slice() {
                (i32::from_str(first).unwrap(),i64::from_str(second).unwrap())
            } else {
                panic!()
            }
        }
    ).map(
        |(first, second)| {
            FileRecord::<i32>{
                label : first,
                size : second
            }
        }
    ).collect();

    let mut map = HashMap::<i32,i64>::new();
    for i in data.clone(){
        if map.contains_key(&i.label){

        } else {
            map.insert(i.label, i.size);
        }
    }

    let data : Vec<FileRecord<i32>> = data.iter().map(|i | {
        let size = map.get(&i.label).unwrap();
        FileRecord::<i32>{
            label: i.label,
            size: *size
        }
    }).collect();


    //let module_names = vec!["wasm_pair_fifo"];

    let mut size : i64 = 512 * 1024 * 4;
    while size < 1024*1024*1024*8 {
        size *= 2;

        let store = Store::default();

        let mut policies: Vec<(&str,Box<dyn PolicyModule<i32>>)> = vec![];
        policies.push((
            "Native FiFo"
             ,Box::new(NativePolicyModule::<FiFo<i32>,i32>::new()
            )
        ));



        let wasm_pair = {
            let path = Path::new("./modules/wasm32-unknown-unknown/release/wasm_pair_fifo.wasm");
            let module = Module::from_file(&store,path).expect("Module Not Found");
            Box::new(WasmPairPolicyModule::from_module(module))
        };

        policies.push(("WASM Pair FiFo",wasm_pair));

        let wasm_bincode = {
            let path = Path::new("./modules/wasm32-unknown-unknown/release/wasm_bincode_fifo.wasm");
            let module = Module::from_file(&store,path).expect("Module Not Found");
            Box::new(WasmBincodePolicyModule::from_module(module))
        };

        policies.push(("WASM Bincode FiFo",wasm_bincode));


        policies.push(("Native GdSize",Box::new(NativePolicyModule::<GdSize<i32>,i32>::new())));

        let wasm_bincode = {
            let path = Path::new("./modules/wasm32-unknown-unknown/release/wasm_pair_gdsize.wasm");
            let module = Module::from_file(&store,path).expect("Module Not Found");
            Box::new(WasmPairPolicyModule::from_module(module))
        };

        policies.push(("WASM Pair GdSize",wasm_bincode));

        let wasm_bincode = {
            let path = Path::new("./modules/wasm32-unknown-unknown/release/wasm_bincode_gdsize.wasm");
            let module = Module::from_file(&store,path).expect("Module Not Found");
            Box::new(WasmBincodePolicyModule::from_module(module))
        };

        policies.push(("WASM Bincode GdSize",wasm_bincode));

        policies.push(("Native LRU",Box::new(NativePolicyModule::<LRU<i32>,i32>::new())));
        let wasm_bincode = {
            let path = Path::new("./modules/wasm32-unknown-unknown/release/wasm_pair_lru.wasm");
            let module = Module::from_file(&store,path).expect("Module Not Found");
            Box::new(WasmPairPolicyModule::from_module(module))
        };

        policies.push(("WASM Pair LRU",wasm_bincode));

        let wasm_bincode = {
            let path = Path::new("./modules/wasm32-unknown-unknown/release/wasm_bincode_lru.wasm");
            let module = Module::from_file(&store,path).expect("Module Not Found");
            Box::new(WasmBincodePolicyModule::from_module(module))
        };

        policies.push(("WASM Bincode LRU",wasm_bincode));

        policies.push(("Native LFU",Box::new(NativePolicyModule::<LFU<i32>,i32>::new())));

        let wasm_bincode = {
            let path = Path::new("./modules/wasm32-unknown-unknown/release/wasm_pair_lfu.wasm");
            let module = Module::from_file(&store,path).expect("Module Not Found");
            Box::new(WasmPairPolicyModule::from_module(module))
        };

        policies.push(("WASM Pair LFU",wasm_bincode));

        let wasm_bincode = {
            let path = Path::new("./modules/wasm32-unknown-unknown/release/wasm_bincode_lfu.wasm");
            let module = Module::from_file(&store,path).expect("Module Not Found");
            Box::new(WasmBincodePolicyModule::from_module(module))
        };

        policies.push(("WASM Bincode LFU",wasm_bincode));
        println!("Size: {0:<10} ",size/(1024*1024));
        for (name, mut policy) in policies {
            let start = std::time::Instant::now();
            policy.initialize(size);
            for file in &data {
                policy.send_request((*file).clone())
            }
            let (total, hits) = policy.stats();
            let end = std::time::Instant::now();
            println!("Name: {0:<20} | Hits: {1:<10} | Time: {2:<10} | Hitrate: {3:<10}", name, hits, (end-start).as_secs_f32(), hits as f32/total as f32 * 100.0);
        }

    }

    //println!("Data: {:?}", &data)





}
