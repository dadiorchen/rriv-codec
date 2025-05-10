fn main() {
}

pub fn encode( timestamp: i64,
               values: &[f64],
               bits: &[u8]
             ) -> Box<[u8]>{
    let mut buffer = Vec::new();
    let mut timestamp = timestamp;
    buffer.push(timestamp as u8);
    for &value in values.iter() {
        let value_bytes = value.to_le_bytes();
        buffer.extend_from_slice(&value_bytes);
    }
    buffer.into_boxed_slice()
}

// write test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let timestamp = 1234567890;
        let values = [1.0, 2.0, 3.0];
        let bits = [1, 2, 3];
        let result = encode(timestamp, &values, &bits);
        assert_eq!(result.len(), 25); // Adjust this based on the expected length of the output
        assert_eq!(result.as_ref()[0], 210); // Check the first byte (timestamp)
        assert_eq!(result.as_ref()[1..9], [0, 0, 0, 0, 0, 0, 0, 0]); // Check the rest of the bytes

    }
}
