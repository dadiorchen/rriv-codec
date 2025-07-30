use chrono::{DateTime, Utc};
use std::mem;
use std::convert::TryInto;
use crate::bits::Bits;
use crate::utils::getBits32;

// offset from 2025-01-01T00:00:00Z, 
// 30 bits to store the offset
// this means we can store timestamps from 2025-01-01T00:00:00Z to 2055-01-01T00:00:00Z
pub fn encode_timestamp(timestamp: i64) -> Bits {
    // Unix timestamp for 2025-01-01T00:00:00Z
    let unix_of_2025 = 1735689600;  
    // Calculate the offset from the Unix timestamp
    let offset = timestamp - unix_of_2025;
    // cut to i32 
    let offset = offset as u32;
    let result = getBits32(offset, 30);
    result
}

pub fn decode_timestamp(bits: &Bits) -> i64 {
    // Ensure the bits are 30 bits long
    assert!(bits.len() == 30, "Bits must be 30 bits long");
    
    let offset = u32::from_str_radix(&bits.to_string(), 2)
        .expect("Failed to convert bits to number");
    
    // Unix timestamp for 2025-01-01T00:00:00Z
    let unix_of_2025 = 1735689600;  
    // Calculate the original timestamp
    unix_of_2025 + offset as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_timestamp() {
        let time_str = "2025-07-28T00:00:00+00:00";
        let datetime = DateTime::parse_from_rfc3339(time_str).expect("Failed to parse date");
        // use i64 to store the timestamp
        let timestamp_i64 = datetime.timestamp();
        let bits = encode_timestamp(timestamp_i64);
        assert_eq!(bits.len(), 30); // Length of the bits
        // Decode the timestamp back
        let decoded_timestamp = decode_timestamp(&bits);
        println!("Decoded timestamp: {}", decoded_timestamp);
        assert_eq!(decoded_timestamp, timestamp_i64);
    }
}
