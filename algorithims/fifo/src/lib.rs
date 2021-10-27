
use simulator_shared_types::CacheEvent;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub struct FiFo<T> {
    cache : Vec<CacheEvent<T>>
}

impl <T> FiFo<T> {
    pub fn initialize(_cache_size : i32) {

    }

    pub fn send(_event : CacheEvent<T>) {

    }
}