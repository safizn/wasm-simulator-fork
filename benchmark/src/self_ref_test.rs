use std::path::Path;
use std::time::Duration;
use ouroboros::self_referencing;
use wasmer::{Function, Instance, Module, Value, imports, Store};

pub fn ouroboros_preload(store: &Store) -> Duration{
    let start = std::time::Instant::now();
    let path = Path::new("./modules/wasm32-unknown-unknown/release/testModule.wasm");
    let module = Module::from_file(store,path).expect("Module Not Found");

    let cached_self_referential = OuroborosCachedFunction::from_module(&module);

    let input = &[Value::I32(2), Value::I32(3)];
    for _i in 1..100_000 {
        let result = cached_self_referential.borrow_function().call(input).expect("Failed to call method: multiply");

        assert_eq!(result[0], Value::I32(6));
    }
    let end = std::time::Instant::now();
    end-start
}

pub fn ouroboros_hotload(store: &Store) -> Duration{
    let start = std::time::Instant::now();
    let path = Path::new("./modules/wasm32-unknown-unknown/release/testModule.wasm");
    let module = Module::from_file(store,path).expect("Module Not Found");

    let cached_self_referential = OuroborosCachedFunction::from_module(&module);

    for _i in 1..100_000 {
        let result = cached_self_referential.borrow_function().call(&[Value::I32(2), Value::I32(3)]).expect("Failed to call method: multiply");

        assert_eq!(result[0], Value::I32(6));
    }
    let end = std::time::Instant::now();
    end-start
}

#[self_referencing]
struct OuroborosCachedFunction {
    module: Instance,
    #[borrows(module)]
    function: &'this Function
}

impl OuroborosCachedFunction {
    fn from_module(module: &Module) -> Self{
        // Prepare environment with imports
        let import_objects = imports!{};
        // Create new sandbox
        let pair_instance = Instance::new(module, &import_objects).expect("Failed to create instance");

        let builder = OuroborosCachedFunctionBuilder{
            module: pair_instance,
            function_builder: |module: &Instance| module.exports.get_function("multiply").expect("Failed to find method: multiply")
        };
        builder.build()

    }

}