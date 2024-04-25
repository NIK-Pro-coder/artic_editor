use std::{fs::File, io::Read};

struct Chunk {
    type_name : String,
    bank : u32,
    data : Vec<u8>,
}

fn read_tic(path: &str, size: u32) -> Vec<Chunk> {
    let mut res = vec![];

    let mut f = File::open(String::from(path)).expect("No file found!");
    let mut buf = vec![0; size as usize];
    let _ = f.read_exact(&mut buf);

    let mut skip = 0;
    let mut edit = 0;

    let mut skip = 0;
    let mut name;

    let mut data = vec![];
    let mut type_name = "".to_string();
    let mut bank = 0u32;

    for (n, i) in buf.iter().enumerate() {
        name = match edit {
            0 => "Bank + Type",
            1 => "Size (little)",
            2 => "Size (big)",
            3 => "Reserved",
            4 => "Chunk data",
            _ => "None",
        }.to_string();
        if edit == 0 {
            bank = (*i >> 5).into();
            let type_id : u32 = (*i as u32 & 0x00011111).into();
            type_name = match type_id {
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
        }
        if edit == 1 { skip += *i as u32 }
        if edit == 2 { skip += (*i as u32) << 8 }
        if edit < 4 { edit += 1 }
        else {
            if skip > 0 { skip -= 1; data.push(*i) }
            if skip < 1 {
                name = "Bank + Type".to_string();
                edit = 1;
                res.push(Chunk{
                    type_name : type_name.clone(),
                    bank : bank,
                    data : data.clone(),
                })
            }
        }
        //println!("{name} {i}");
    }

    res
}

fn main() -> () {
    let file = read_tic("test.tic", 1191);

    for (n, i) in file.iter().enumerate() {
        println!("{} chunk in bank {}",i.type_name,i.bank);
    }
}
