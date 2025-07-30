use crate::bits::Bits;

pub fn number_to_binary_array(num: u16, bits: usize) -> Vec<u8> {
    println!("Converting number {} to binary array with {} bits", num, bits);
    // get binary from highest bit to lowest
    let bi_s = format!("{:0width$b}", num, width = 16);
    println!("Binary string: {}", bi_s);
    bi_s
        .chars()
        .map(|c| c.to_digit(2).unwrap() as u8)
        .collect::<Vec<u8>>()
        .into_iter()
        .take(bits)
        .collect()
}

pub fn number_to_binary_array_32(num: u32, bits: usize) -> Vec<u8> {
    println!("Converting number {} to binary array with {} bits", num, bits);
    // get binary from highest bit to lowest
    let bi_s = format!("{:0width$b}", num, width = 32);
    println!("Binary string: {}", bi_s);
    bi_s
        .chars()
        .map(|c| c.to_digit(2).unwrap() as u8)
        .collect::<Vec<u8>>()
        .into_iter()
        .take(bits)
        .collect()
}

pub fn number_to_binary_array_8(num: u8, bits: usize) -> Vec<u8> {
    println!("Converting number {} to binary array with {} bits", num, bits);
    // get binary from highest bit to lowest
    let bi_s = format!("{:0width$b}", num, width = 8);
    println!("Binary string: {}", bi_s);
    bi_s
        .chars()
        .map(|c| c.to_digit(2).unwrap() as u8)
        .collect::<Vec<u8>>()
        .into_iter()
        .take(bits)
        .collect()
}

pub fn getBits(num: u16, bits: usize) -> Bits {
    let shifted_temp = num << (16 - bits);
    Bits::new(number_to_binary_array(shifted_temp , bits).into_boxed_slice())
}

pub fn get_bits_8(num: u8, bits: usize) -> Bits {
    let shifted_temp = num << (8 - bits);
    Bits::new(number_to_binary_array_8(shifted_temp , bits).into_boxed_slice())
}

pub fn getBits32(num: u32, bits: usize) -> Bits {
    let shifted_temp = num << (32 - bits);
    Bits::new(number_to_binary_array_32(shifted_temp , bits).into_boxed_slice())
}


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
