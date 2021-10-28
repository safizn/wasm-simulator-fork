use wasmer::{Store, Module, Instance, imports, Value, Val, Function};
use simulator_shared_types::FileRecord;

pub struct WasmBincodePolicyModule {
    module : Instance
}

pub struct WasmPairPolicyModule {
    module : Instance,
}

pub trait PolicyModule<T> {

    fn initialize(&mut self, cache_size : i64);

    fn send_request(&mut self, pair : FileRecord<T>);

    fn stats(&self) -> (i32, i32);
}

impl WasmBincodePolicyModule {
    fn alloc(&self, size : i32) -> (i32,i32) {
        let results = &self.module.exports.get_function("alloc").unwrap().call(&[Val::I32(size)]).unwrap();
        let merged = results[0].unwrap_i64();
        packed_i32::split_i64_to_i32(merged)
    }

    pub fn from_module(module : Module) -> Self {
        let import_objects = imports!{};
        // Create new sandbox
        let module = Instance::new(&module, &import_objects).unwrap();

        let _alloc = module.exports.get_function("alloc").unwrap();
        let _send = module.exports.get_function("send").unwrap();
        let _init = module.exports.get_function("init").unwrap();
        let _init = module.exports.get_function("stats").unwrap();



        let out = WasmBincodePolicyModule {
            module
        };

        out

    }
}

impl WasmPairPolicyModule {

    pub fn from_module(module : Module) -> Self {
        let import_objects = imports!{};
        // Create new sandbox
        let module = Instance::new(&module, &import_objects).unwrap();
        let _send = module.exports.get_function("send").unwrap();
        let _init = module.exports.get_function("init").unwrap();
        let _init = module.exports.get_function("stats").unwrap();

        let out = WasmPairPolicyModule {
            module
        };
        out
    }
}

impl PolicyModule<i32> for WasmPairPolicyModule {
    fn initialize(&mut self, cache_size: i64) {
        self.module.exports.get_function("init").unwrap().call(&[Val::I64(cache_size)]).unwrap();
    }
    fn send_request(&mut self, request : FileRecord<i32>){
        self.module.exports.get_function("send").unwrap().call(&[Val::I32(request.label),Val::I64(request.size)]).unwrap();
    }

    fn stats(&self) -> (i32, i32) {
        let result = self.module.exports.get_function("stats").unwrap().call(&[]).unwrap();
        let packed = result[0].i64().unwrap();
        packed_i32::split_i64_to_i32(packed)
    }
}

impl PolicyModule<i32> for WasmBincodePolicyModule {
    fn initialize(&mut self, cache_size: i64) {
        self.module.exports.get_function("init").unwrap().call(&[Val::I64(cache_size)]).unwrap();
    }
    fn send_request(&mut self, request: FileRecord<i32>) {
        let buffer_size = bincode::serialized_size(&request).expect("Could not calculate buffer size") as i32;

        let (ptr, len) = self.alloc(buffer_size);

        let mem = self.module.exports.get_memory("memory").expect("Could not get memory");
        let mem_array: &mut [u8];
        let serialized_array = bincode::serialize(&request).expect("Failed to serialize");
        unsafe {
            mem_array = mem.data_unchecked_mut(); // Set base address to memory
            for i in 0..len {
                // iterate over the serialized struct, copying it to the memory of the target module,
                // using the ptr provided by prepare_buffer
                mem_array[ptr as usize + i as usize] = serialized_array[i as usize];
            }
        }
        self.module.exports.get_function("send").unwrap().call(&[Val::I32(ptr), Val::I32(len)]).unwrap();
    }

    fn stats(&self) -> (i32, i32) {
        let result = self.module.exports.get_function("stats").unwrap().call(&[]).unwrap();
        let packed = result[0].i64().unwrap();
        packed_i32::split_i64_to_i32(packed)
    }
}
