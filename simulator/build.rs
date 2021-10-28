use std::process::Command;

// Build script to compile and package WASM modules for executable

const WASM_UNKNOWN: &str = "wasm32-unknown-unknown";

fn main(){
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=simulator_shared_types/*");

    let modules = vec![
        "sim_modules/wasm_bincode/wasm_bincode_fifo", "sim_modules/wasm_c_struct/wasm_c_fifo","sim_modules/wasm_pair/wasm_pair_fifo",

    ];

    for module in &modules {
        println!("cargo:rerun-if-changed={}/*",module);
    }



    for module in modules {
        let _result = Command::new("cargo")
            .args(&["build",format!("--target={}",WASM_UNKNOWN).as_str(),"--target-dir=../../../modules","--release"])
            .current_dir(format!("../{}",module))
            .status()
            .expect(format!("Compilation Failed for wasm module:{}",module).as_str());
        // TODO copy file to output
    }
}