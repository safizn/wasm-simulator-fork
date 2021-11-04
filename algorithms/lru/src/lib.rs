use std::cmp::Reverse;
use std::hash::Hash;
use priority_queue::priority_queue::PriorityQueue;
use algorithm::CacheAlgorithm;
use simulator_shared_types::FileRecord;

pub struct LRU<T> where T : Hash + Eq {
    heap: PriorityQueue<FileRecord<T>, Reverse<u64>>,
    current_used : i64, // current space in cache
    size : i64, // size of cache
    event_count:u64,
    hit_count : i32,
}

impl<T> CacheAlgorithm<T> for LRU<T> where T : Hash + Eq + Clone{
    fn simulate(&mut self, file: FileRecord<T>) {
        if file.size > self.size {
            println!("FILE SIZE: {}", file.size);
            panic!("File larger than cache")
        }
        self.event_count += 1;
        if let Some(_) = self.heap.change_priority(&file, Reverse(self.event_count)) {
            self.hit_count += 1;
            return;
        }

        //self.cache.insert(file.label.clone());
        self.current_used += file.size;

        self.heap.push(file, Reverse(self.event_count)); // use event count as
        while self.current_used > self.size {
            let popped = self.heap.pop().unwrap();
            self.current_used -= popped.0.size;
        }
    }


    fn new(size: i64) -> Self {
        LRU::<T> {
            heap: PriorityQueue::<FileRecord<T>, Reverse<u64>>::new(),
            current_used: 0,
            size,
            event_count: 0,
            hit_count: 0
        }
    }

    fn stats(&self) -> (i32, i32) {
        (self.event_count as i32, self.hit_count)
    }
}
