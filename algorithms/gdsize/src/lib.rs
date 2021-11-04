use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::hash::Hash;
use algorithm::CacheAlgorithm;
use simulator_shared_types::FileRecord;

pub struct GdSize<T>{
    heap: BinaryHeap<SortedFileRecord<T>>,
    cache: HashSet<T>,
    current_used : i64, // current space in cache
    size : i64, // size of cache
    event_count: i32,
    hit_count : i32,
    inflation : f64
}
/*
 New type pattern to implement sorting for shared type.
 */

#[derive(PartialEq, PartialOrd)]
struct NonNan(f64);

impl Eq for NonNan{}

impl Ord for NonNan{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
#[derive(Eq)]
struct SortedFileRecord<T>{
    record : FileRecord<T>,
    h_value: NonNan
}

impl <T> PartialEq<Self> for SortedFileRecord<T> where T : Eq {
    fn eq(&self, other: &Self) -> bool {
        self.record.size == other.record.size
    }
}

impl <T> PartialOrd<Self> for SortedFileRecord<T> where T : Eq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl <T> Ord for SortedFileRecord<T> where T : Eq{
    fn cmp(&self, other: &Self) -> Ordering {
        other.h_value.cmp(&self.h_value)
        // other.record.size.cmp(&self.record.size)
    }
}

impl <T> GdSize<T> where T : Hash + Eq + Clone {
    fn in_cache(&self, file : &FileRecord<T>) -> bool {
        self.cache.contains(&file.label.clone())
    }
}

impl<T> CacheAlgorithm<T> for GdSize<T> where T : Hash + Eq + Clone{
    fn simulate(&mut self, file: FileRecord<T>) {
        if file.size > self.size {
            println!("FILE SIZE: {}", file.size);
            panic!("File larger than cache")
        }
        self.event_count += 1;
        if self.in_cache(&file) {
            self.hit_count += 1;
            return;
        } else {
            let id = file.label.clone();
            self.current_used += file.size;
            while self.current_used > self.size {
                let popped = self.heap.pop().unwrap();
                if popped.record.label == id {
                    panic!("Popped file we just inserted")
                }
                self.cache.remove(&popped.record.label.clone());
                self.current_used -= popped.record.size;
            }
        }

        self.cache.insert(file.label.clone());
        self.current_used += file.size;

        let h_value = self.inflation + 1.0/(file.size as f64);
        let sorted = SortedFileRecord{
            record: file,
            h_value: NonNan(h_value)
        };
        self.heap.push(sorted);


    }


    fn new(size: i64) -> Self {
        GdSize::<T> {
            heap: BinaryHeap::<SortedFileRecord<T>>::new(),
            cache: Default::default(),
            current_used: 0,
            size,
            event_count: 0,
            hit_count: 0,
            inflation: 0.0
        }
    }

    fn stats(&self) -> (i32, i32) {
        (self.event_count, self.hit_count)
    }
}
