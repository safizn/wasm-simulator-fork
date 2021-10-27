use wasmer::{Store, Module, Instance, imports, Value, Val, Function};
use simulator_shared_types::FileRecord;

pub struct WasmStructPolicy{
    module : Instance
}

pub struct WasmPairPolicy{
    module : Instance,
}

pub trait Policy<T> {

    fn initialize(&mut self, cache_size : i32);

    fn send_request(&mut self, pair : FileRecord<T>);

    fn state(&self) -> (i32, i32);
}

impl WasmStructPolicy {
    fn alloc(&self, size : i32) -> (i32,i32) {
        let results = &self.module.exports.get_function("alloc").unwrap().call(&[Val::I32(size)]).unwrap();
        let merged = results[0].unwrap_i64();
        packed_i32::split_i64_to_i32(merged)
    }

    fn from_module(module : &Module) -> Self {
        let import_objects = imports!{};
        // Create new sandbox
        let module = Instance::new(&module, &import_objects).unwrap();

        let _alloc = module.exports.get_function("alloc").unwrap();
        let _send = module.exports.get_function("send").unwrap();
        let _init = module.exports.get_function("init").unwrap();



        let out = WasmStructPolicy {
            module
        };

        out

    }
}

impl WasmPairPolicy {

    fn from_module(module : Module) -> Self {
        let import_objects = imports!{};
        // Create new sandbox
        let module = Instance::new(&module, &import_objects).unwrap();
        let _send = module.exports.get_function("send").unwrap();
        let _init = module.exports.get_function("init").unwrap();

        let out = WasmPairPolicy {
            module
        };
        out
    }
}

impl Policy<i32> for WasmPairPolicy {
    fn initialize(&mut self, cache_size: i32) {
        &self.module.exports.get_function("init").unwrap().call(&[Val::I32(cache_size)]).unwrap();
    }
    fn send_request(&mut self, request : FileRecord<i32>){
        &self.module.exports.get_function("send").unwrap().call(&[Val::I32(request.label),Val::I32(request.size)]).unwrap();
    }

    fn state(&self) -> (i32, i32) {
        todo!()
    }
}

impl Policy<i32> for WasmStructPolicy {
    fn initialize(&mut self, cache_size: i32) {
        &self.module.exports.get_function("init").unwrap().call(&[Val::I32(cache_size)]).unwrap();
    }
    fn send_request(&mut self, request : FileRecord<i32>){
        todo!()
    }

    fn state(&self) -> (i32, i32) {
        todo!()
    }
}