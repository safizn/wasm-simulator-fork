use flate2::read::GzDecoder;
use std::fs::File;
use std::io::Read;
use tar::{Archive, Entry};

fn main() {
    let file = File::open("./datasets/test.dat.gz").unwrap();
    let mut decompressed = GzDecoder::new(file);



    let mut string = String::new();
    decompressed.read_to_string(&mut string).unwrap();
    println!("Dat file contents: \n {}",string)



}
