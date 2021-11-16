use std::process::Command;

// Build script to compile and package WASM modules for executable

const WASM_UNKNOWN: &str = "wasm32-unknown-unknown";

fn main(){

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=benchmark_shared_data_structures/*");
    let modules = vec!["testModule", "loop_test_module","struct_addition"];

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
        // TODO copy file to output
    }
}