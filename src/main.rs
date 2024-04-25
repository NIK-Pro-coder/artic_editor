use std::{fs::File, io::Read};

struct Chunk {
    type_name : String,
    type_id : u32,
    size : u32,
    bank : u32,
    data : Vec<u8>,
}

fn read_chunk(data: &Vec<u8>, offset: usize) -> Chunk {
    let location : u32 = (data[offset] >> 5).into();
    let id : u32 = (data[offset] as u32 & 0x00011111).into();
    let name = match id {
        1 => "Tiles",
        2 => "Sprites",
        4 => "Map",
        5 => "Code",
        6 => "Flags",
        9 => "Samples",
        10 => "Waveform",
        12 => "Palette",
        14 => "Music",
        15 => "Patterns",
        17 => "Default",
        18 => "Screen",
        19 => "Binary",
        _ => "(Reserved)"
    }.to_string();
    let big : u32 = (data[offset + 1] as u32 + (data[offset + 2] as u32) << 8).into();
    let mut storage = vec![];

    for i in offset + 4..offset + 4 + big as usize {
        storage.push(data[i]);
    }

    println!("{name} chunk located at {offset} in bank {location} with size {big}");

    Chunk {
        type_name : name,
        type_id : id,
        size : big,
        bank : location,
        data : storage,
    }
}

fn read_tic(data: Vec<u8> ) -> Vec<Chunk> {
    let mut res = vec![];

    let mut skip = 0;

    let mut v = [0u8, 0u8, 0u8];
    let mut id = 0;

    for (n, i) in data.iter().enumerate() {
        if skip > 0 {
            skip -= 1;
        } else {
            v[id] = *i;
            id += 1;
            if id > 2 {
                let pos : u16 = (v[1] as u16).into();
                res.push(read_chunk(&data, pos.into()));
                id = 0;
            }
        }
    }

    return res;
}

fn main() -> () {
    let count = 1191;

    let mut f = File::open("test.tic").expect("No file found!");
    let mut buf = vec![0; count];
    let _ = f.read_exact(&mut buf);
    for (n, i) in buf.iter().enumerate() {
        println!("{i}");
    }
    //read_tic(buf);
}
