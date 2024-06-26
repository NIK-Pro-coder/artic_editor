
use std::{fs::File, io::Read, thread::sleep, time::Duration};

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

    let mut data = vec![];
    let mut type_name = "".to_string();
    let mut type_id = 0;
    let mut bank = 0u32;

    let mut zero_extended = false;

    for (_n, i) in buf.iter().enumerate() {
        sleep(Duration::from_millis(10));
        println!("{i} {edit} {skip} {zero_extended}");
        if edit > 3 {
            if skip > 0 {
                skip -= 1;
                data.push(i.clone());
                continue
            } else if zero_extended && *i == 0 {
                data.push(i.clone());
                continue
            } else {
                edit = 0;
                res.push(Chunk{
                    type_name : type_name.clone(),
                    bank : bank,
                    data : data.clone(),
                });
                data.clear();
            }
        }
        type_id = match edit {
            0 => (*i as u32 & 0x00011111) as u32,
            _ => type_id,
        };
        zero_extended = match type_id {
            1 | 2 | 4 => true,
            _ => false,
        };
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
        if edit == 0 {
            println!("{type_name}");
        }
        bank = match edit {
            0 => (*i >> 5).into(),
            _ => bank,
        };
        skip += match edit {
            1 => *i as u32,
            2 => (*i as u32) << 8,
            _ => 0,
        };
        edit += 1;
    }
    res.push(Chunk{
        type_name : type_name.clone(),
        bank : bank,
        data : data.clone(),
    });

    res
}

fn main() -> () {
    let file = read_tic("test.tic", 9528);

    for (_n, i) in file.iter().enumerate() {
        println!("{} chunk in bank {}",i.type_name,i.bank,);
    }
}
