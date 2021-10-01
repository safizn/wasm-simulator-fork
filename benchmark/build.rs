use std::process::Command;

// Build script to compile and package WASM modules for executable

const WASM_UNKNOWN: &str = "wasm32-unknown-unknown";

fn main(){

    println!("cargo:rerun-if-changed=testModule/*");
    println!("cargo:rerun-if-changed=loop_test_module/*");
    let modules = vec!["testModule", "loop_test_module"];



    for module in modules {
        let _result = Command::new("cargo")
            .args(&["build",format!("--target={}",WASM_UNKNOWN).as_str(),"--target-dir=../modules"])
            .current_dir(format!("../{}",module))
            .status()
            .expect(format!("Compilation Failed for wasm module:{}",module).as_str());
        // TODO copy file to output
    }
}