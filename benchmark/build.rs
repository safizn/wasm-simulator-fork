use std::process::Command;

// Build script to compile and package WASM modules for executable

const WASM_UNKNOWN: &str = "wasm32-unknown-unknown";

fn main(){

    let modules = vec!["testModule"];



    for module in modules {
        let _result = Command::new("cargo")
            .args(&["build",format!("--target={}",WASM_UNKNOWN).as_str(),"--target-dir=../target_wasm"])
            .current_dir(format!("../{}",module))
            .status()
            .expect(format!("Compilation Failed for wasm module:{}",module).as_str());
        // TODO copy file to output
    }
}