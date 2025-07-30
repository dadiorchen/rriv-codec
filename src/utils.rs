
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
