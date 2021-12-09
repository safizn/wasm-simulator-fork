use std::time::Duration;
use wasmer::Store;

pub fn native_test(_ : &Store) -> Duration{
    let start = std::time::Instant::now();
    for i in 1..100_000 {
        let result = 2*i; //slightly different to prevent release mode from deleting this test entirely

        assert_eq!(result,2*i);
    }
    let end = std::time::Instant::now();
    end-start
}