// print hex readable
pub fn hex_dump(data: &[u8]) {
    for (i, chunk) in data.chunks(16).enumerate() {
        print!("{:08x}  ", i * 16);
        for byte in chunk {
            print!("{:02x} ", byte);
        }
        // Print ASCII representation
        print!(" {:>width$}", "", width = (16 - chunk.len()) * 3);
        for byte in chunk {
            let c = if (0x20..0x7F).contains(byte) {
                *byte as char
            } else {
                '.'
            };
            print!("{}", c);
        }
        println!();
    }
}

