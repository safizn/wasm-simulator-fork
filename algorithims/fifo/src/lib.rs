use std::collections::{ HashSet, VecDeque};
use std::hash::Hash;
use simulator_shared_types::FileRecord;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub struct FiFo<T> {
    queue : VecDeque<FileRecord<T>>, // Double ended queue - basically ring buffer for order items have entered queue
    cache : HashSet<FileRecord<T>>,
    current_used : i32, // current space in cache
    size : i32, // size of cache
    event_count: i32,
    hit_count : i32
}

impl <T> FiFo<T> where T : Hash + Eq{
    pub fn simulate(&mut self, file: FileRecord<T>) {
        self.event_count += 1;
        if self.in_cache(file) {
            return;
        }


    }

    fn in_cache(&self, file : FileRecord<T>) -> bool {
        self.cache.contains(&file)
    }


    pub fn new(size: i32) -> Self {
        FiFo::<T> {
            queue: VecDeque::<FileRecord<T>>::new(),
            cache: Default::default(),
            current_used: 0,
            size,
            event_count: 0,
            hit_count: 0
        }
    }

    pub fn stats(&self) -> (i32,i32) {
        (self.event_count, self.hit_count)
    }
}