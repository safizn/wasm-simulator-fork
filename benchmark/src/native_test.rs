use std::time::Duration;
use wasmer::Store;

pub fn native_test(_ : &Store) -> Duration{
    let start = std::time::Instant::now();
    for _i in 1..100_000 {
        let result = 2*3;

        assert_eq!(result,6);
    }
    let end = std::time::Instant::now();
    end-start
}