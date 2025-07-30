use chrono::{DateTime, Utc};
use std::mem;
use std::convert::TryInto;
use crate::bits::Bits;
use crate::utils::{getBits32, get_bits_8};

// humidity range: 0 to 100
// need 7 bits
pub fn encode_humidity(humidity: u8) -> Bits {
    let result = get_bits_8(humidity, 7);
    result
}

pub fn decode_humidity(bits: &Bits) -> u8 {
    // Ensure the bits are 30 bits long
    assert!(bits.len() == 7, "Bits must be 7 bits long");
    
    let offset = u8::from_str_radix(&bits.to_string(), 2)
        .expect("Failed to convert bits to number");
    // Return the humidity value
    offset
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_humidity() {
        let humidity = 50;
        let bits = encode_humidity(humidity);
        assert_eq!(bits.len(), 7);
        let decoded_timestamp = decode_humidity(&bits);
        println!("Decoded timestamp: {}", decoded_timestamp);
        assert_eq!(decoded_timestamp, humidity);
    }
}
