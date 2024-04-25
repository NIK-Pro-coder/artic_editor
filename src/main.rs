use std::{fs::File, io::{self, Read, Seek, SeekFrom}};

struct chunk {
    r#type : u32,
    size : u32,
    bank : u8,
    data : Vec<u8>,
}

fn read_chunk(data: Vec<u8>, offset: u32) -> () {}

fn read_tic(data: Vec<u8> ) -> () {
    u32 skip = 0;
}

fn main() -> () {
    println!("Hello, world!");

    let start = 0;
    let count = 4;

    let mut f = File::open("test.tic").expect("No file found!");
    f.seek(SeekFrom::Start(start));
    let mut buf = vec![0; count];
    f.read_exact(&mut buf);
    read_tic(buf)
}
