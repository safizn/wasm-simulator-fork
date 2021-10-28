use once_cell::sync::Lazy;
use std::sync::Mutex;
use fifo::FiFo;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


static POLICY : Lazy<Mutex<Option<FiFo<i32>>>> = Lazy::new(||{
   Mutex::new(None)
});

fn init(size: i64){
    POLICY.lock().unwrap().insert(
        FiFo::<i32>::new(size)
    );
}