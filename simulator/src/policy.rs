use wasmer::{Store, Module, Instance, imports, Value, Val, Function};

pub struct WasmPolicy{
    module : Instance,
    alloc : Function,
    send : Function
}



pub trait Policy {
    fn send_request(&self, pair : (i32,i32));
}

impl WasmPolicy {
    fn alloc(&self, size : i32) -> (i32,i32) {
        let results = &self.alloc.call(&[Val::I32(size)]).unwrap();
        let merged = results[0].unwrap_i64();
        packed_i32::split_i64_to_i32(merged)
    }
}

impl Policy for WasmPolicy {
    fn send_request(&self, (item, size) : (i32,i32)){
        &self.send.call(&[Val::I32(item),Val::I32(size)]);
    }
}