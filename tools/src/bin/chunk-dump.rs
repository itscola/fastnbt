use std::io::Write;

use fastanvil::{Chunk, Region};

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let file = std::fs::File::open(args[0].clone()).unwrap();

    let mut region = Region::new(file);

    region
        .for_each_chunk(|x, z, data| {
            let mut file = std::fs::File::create(format!("chunks/{}.{}.nbt", x, z)).unwrap();
            file.write_all(data).unwrap();

            let mut file = std::fs::File::create(format!("chunks/{}.{}.txt", x, z)).unwrap();
            let chunk: Chunk = fastnbt::de::from_bytes(data).unwrap();
            file.write_all(format!("{:#?}", chunk).as_bytes()).unwrap();
        })
        .unwrap();
}
