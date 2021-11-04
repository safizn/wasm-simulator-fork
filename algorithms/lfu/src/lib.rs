use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use priority_queue::priority_queue::PriorityQueue;
use algorithm::CacheAlgorithm;
use simulator_shared_types::FileRecord;

pub struct LFU<T> where T : Hash + Eq{
    heap: PriorityQueue<FileRecord<T>,Reverse<FileSorting>>,
    memory : HashMap<FileRecord<T>,u64>,
    current_used : i64, // current space in cache
    size : i64, // size of cache
    event_count: u64,
    hit_count : i32,
}
/*
 New type pattern to implement sorting for shared type.
 */


impl PartialOrd<Self> for FileSorting {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FileSorting {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.freq.cmp(&other.freq) {
            Ordering::Less => {Ordering::Less}
            Ordering::Equal => {
                self.last_used.cmp(&other.last_used)
            }
            Ordering::Greater => {Ordering::Greater}
        }
    }
}

#[derive(Eq,PartialEq,Debug,Clone)]
struct FileSorting {
    freq : u64,
    last_used: u64
}


impl<T> CacheAlgorithm<T> for LFU<T> where T : Hash + Eq + Clone + Debug{
    fn simulate(&mut self, file: FileRecord<T>) {
        if file.size > self.size {
            println!("FILE SIZE: {}", file.size);
            panic!("File larger than cache")
        }
        self.event_count += 1;
        let size = file.size;

        let prev = match self.heap.get_priority(&file) {
            Some(i) => i.0.freq + 1_u64,
            None => match self.memory.get(&file) {
                None =>  1_u64,
                Some(i) => i + 1_u64
            }
        };

        let id = file.label.clone();
        if let None = self.heap.get_priority(&file){
            self.current_used += size;
            while self.current_used > self.size {
                let popped = self.heap.pop().unwrap();
                if popped.0.label == id {
                    panic!("Popped file we just inserted")
                }
                //println!("POPPED: {:?}", popped);
                self.current_used -= popped.0.size;
                self.memory.insert(popped.0, popped.1.0.freq);
            }
        }


        let new = FileSorting{
            freq: prev,
            last_used: self.event_count
        };

        if let Some(z) = self.heap.push(file,Reverse(new.clone())){
            self.hit_count += 1;
            return;
        }



    }


    fn new(size: i64) -> Self {
        LFU::<T> {
            heap: PriorityQueue::<FileRecord<T>,Reverse<FileSorting>>::new(),
            memory: HashMap::new(),
            current_used: 0,
            size,
            event_count: 0,
            hit_count: 0,
        }
    }

    fn stats(&self) -> (i32, i32) {
        (self.event_count as i32, self.hit_count)
    }
}
