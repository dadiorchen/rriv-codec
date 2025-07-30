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


