#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[no_mangle]
pub fn multiply_many_times(x: i32, y :i32, times: i32) -> i32{
    let mut result = 0;
    for _i in 1..times{
        result = x * y
    }
    result
}
