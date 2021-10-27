use flate2::read::GzDecoder;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use clap::{App, Arg};
use simulator_shared_types::FileRecord;

mod policy;
mod native_modules;


fn main() {

    let matches = App::new("Caching Policy Simulator")
        .version("0.1")
        .author("Devon Hockley")
        .about("A caching policy simulator implemented using WASM")
        .arg(Arg::with_name("sample")
            .help("Sets the input data sample")
            .index(1)
            .required(true)
        )
        .get_matches();

    let file_path = matches.value_of("sample").unwrap();

    println!("Using input file: {}", file_path);



    let file = File::open(file_path).unwrap();
    let mut decompressed = GzDecoder::new(file);



    let mut string = String::new();
    decompressed.read_to_string(&mut string).unwrap();

    let data : Vec<FileRecord<i32>> = string.lines().map(
        |line| {
            if let [first, second, ..] = line.trim().split_ascii_whitespace().collect::<Vec<&str>>().as_slice() {
                (i32::from_str(first).unwrap(),i32::from_str(second).unwrap())
            } else {
                panic!()
            }
        }
    ).map(
        |(first, second)| {
            FileRecord::<i32>{
                label : first,
                size : second
            }
        }
    ).collect();


    let module_names = vec!["wasm_pair_fifo"];



    println!("Data: {:?}", &data)





}
