#[cfg(test)]
mod tests {
    use crate::multiply;

    #[test]
    fn it_works() {
        assert_eq!(multiply(2,2), 4);
    }
}

// Basic function call using WASM native types
#[no_mangle] // Tells rustc not to mangle the symbol, required for host to find method
pub fn multiply(x : i32, y :i32 ) -> i32 {
    return x * y
}