use std::process::Command;

// Build script to compile and package WASM modules for executable

const WASM_UNKNOWN: &str = "wasm32-unknown-unknown";

fn main(){
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=simulator_shared_types/*");

    let modules = vec![
        "sim_modules/wasm_bincode/wasm_bincode_fifo", "sim_modules/wasm_c_struct/wasm_c_fifo","sim_modules/wasm_pair/wasm_pair_fifo",
        "sim_modules/wasm_bincode/wasm_bincode_gdsize","sim_modules/wasm_c_struct/wasm_c_gdsize", "sim_modules/wasm_pair/wasm_pair_gdsize",
        "sim_modules/wasm_bincode/wasm_bincode_lfu","sim_modules/wasm_c_struct/wasm_c_lfu", "sim_modules/wasm_pair/wasm_pair_lfu",
        "sim_modules/wasm_bincode/wasm_bincode_lru","sim_modules/wasm_c_struct/wasm_c_lru", "sim_modules/wasm_pair/wasm_pair_lru",
    ];

    for module in &modules {
        println!("cargo:rerun-if-changed={}/*",module);
    }



    for module in modules {
        let _result = Command::new("cargo")
            .args(&["build",format!("--target={}",WASM_UNKNOWN).as_str(),"--target-dir=../../../modules","--release"])
            .current_dir(format!("../{}",module))
            .status().unwrap();

        if _result.code().unwrap() != 0 {
            panic!("Compilation error for module: {}", module)
        }

    }
}