use wasmer::{Function, Instance, Memory, Module, Val, imports};
use ouroboros::self_referencing;
use simulator_shared_types::FileRecord;
use crate::policy::PolicyModule;

#[self_referencing]
pub struct WasmCachedBincodePolicyModule{
    module : Instance,
    #[borrows(module)]
    mem: &'this Memory,
    #[borrows(module)]
    alloc: &'this Function,
    #[borrows(module)]
    send: &'this Function,
    #[borrows(module)]
    init: &'this Function,
    #[borrows(module)]
    stats: &'this Function,
}

#[self_referencing]
pub struct WasmCachedBytemuckPolicyModule{
    module : Instance,
    ptr: usize,
    len: usize,
    #[borrows(module)]
    mem: &'this Memory,
    #[borrows(module)]
    alloc: &'this Function,
    #[borrows(module)]
    send: &'this Function,
    #[borrows(module)]
    init: &'this Function,
    #[borrows(module)]
    stats: &'this Function,
}

impl WasmCachedBincodePolicyModule {
    fn alloc(&self, size : i32) -> (i32,i32) {
        let results = &self.borrow_alloc().call(&[Val::I32(size)]).unwrap();
        let merged = results[0].unwrap_i64();
        packed_i32::split_i64_to_i32(merged)
    }

    pub fn from_module(module : Module) -> Self {
        let import_objects = imports!{};
        // Create new sandbox
        let module = Instance::new(&module, &import_objects).unwrap();

        let mem = module.exports.get_memory("memory").expect("Could not get memory");

        let alloc = module.exports.get_function("alloc").unwrap();
        let send = module.exports.get_function("send").unwrap();
        let init = module.exports.get_function("init").unwrap();
        let stats = module.exports.get_function("stats").unwrap();



        let out = WasmCachedBincodePolicyModuleBuilder {
            module,
            mem_builder: |module: &Instance| module.exports.get_memory("memory").expect("Could not get memory"),
            alloc_builder: |module: &Instance| module.exports.get_function("alloc").unwrap(),
            send_builder: |module: &Instance| module.exports.get_function("send").unwrap(),
            init_builder: |module: &Instance| module.exports.get_function("init").unwrap(),
            stats_builder: |module: &Instance| module.exports.get_function("stats").unwrap()
        };

        out.build()

    }
}

impl PolicyModule<i32> for WasmCachedBincodePolicyModule {
    fn initialize(&mut self, cache_size: i64) {
        self.borrow_init().call(&[Val::I64(cache_size)]).unwrap();
    }
    fn send_request(&mut self, request: FileRecord<i32>) {
        let buffer_size = bincode::serialized_size(&request).expect("Could not calculate buffer size") as i32;

        let (ptr, len) = self.alloc(buffer_size);

        let mem = self.borrow_mem();
        runtime::write_bincode_to_wasm_memory(request, mem, ptr as usize, len as usize);
        // let mem_array: &mut [u8];
        // let serialized_array = bincode::serialize(&request).expect("Failed to serialize");
        // unsafe {
        //     mem_array = mem.data_unchecked_mut(); // Set base address to memory
        //     for i in 0..len {
        //         // iterate over the serialized struct, copying it to the memory of the target module,
        //         // using the ptr provided by prepare_buffer
        //         mem_array[ptr as usize + i as usize] = serialized_array[i as usize];
        //     }
        // }
        self.borrow_send().call(&[Val::I32(ptr), Val::I32(len)]).unwrap();
    }

    fn stats(&self) -> (i32, i32) {
        let result = self.borrow_stats().call(&[]).unwrap();
        let packed = result[0].i64().unwrap();
        packed_i32::split_i64_to_i32(packed)
    }
}

impl WasmCachedBytemuckPolicyModule {
    pub fn from_module(module : Module) -> Self {
        let import_objects = imports!{};
        // Create new sandbox
        let module = Instance::new(&module, &import_objects).unwrap();

        let mem = module.exports.get_memory("memory").expect("Could not get memory");

        let alloc = module.exports.get_function("alloc").unwrap();
        let send = module.exports.get_function("send").unwrap();
        let init = module.exports.get_function("init").unwrap();
        let stats = module.exports.get_function("stats").unwrap();

        let buffer_size = std::mem::size_of::<FileRecord<i32>>();

        let (ptr, len) = {
            let results = alloc.call(&[Val::I32(buffer_size as i32)]).unwrap();
            let merged = results[0].unwrap_i64();
            packed_i32::split_i64_to_i32(merged)
        };

        let out = WasmCachedBytemuckPolicyModuleBuilder {
            module,
            ptr: ptr as usize,
            len: len as usize,
            mem_builder: |module: &Instance| module.exports.get_memory("memory").expect("Could not get memory"),
            alloc_builder: |module: &Instance| module.exports.get_function("alloc").unwrap(),
            send_builder: |module: &Instance| module.exports.get_function("send").unwrap(),
            init_builder: |module: &Instance| module.exports.get_function("init").unwrap(),
            stats_builder: |module: &Instance| module.exports.get_function("stats").unwrap()
        };

        out.build()

    }
}

impl PolicyModule<i32> for WasmCachedBytemuckPolicyModule {
    fn initialize(&mut self, cache_size: i64) {
        self.borrow_init().call(&[Val::I64(cache_size)]).unwrap();
    }
    fn send_request(&mut self, request: FileRecord<i32>) {
        let mem = self.borrow_mem();
        let ptr = self.borrow_ptr();
        let len = self.borrow_len();
        runtime::write_bytemuck_to_wasm_memory(request, mem, *ptr, *len);
        self.borrow_send().call(&[Val::I32(*ptr as i32), Val::I32(*len as i32)]).unwrap();
    }

    fn stats(&self) -> (i32, i32) {
        let result = self.borrow_stats().call(&[]).unwrap();
        let packed = result[0].i64().unwrap();
        packed_i32::split_i64_to_i32(packed)
    }
}
