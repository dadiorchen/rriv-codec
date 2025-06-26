fn main() {
}

pub fn encode( 
    timestamp: i64,
    // The temprature is a 8 element array, every one is 64-bit float
    temperature: [f64; 6],
    // The humidity is a 64-bit float
    humidity: f64,
) -> Box<[f64]> {
    // Create a buffer to hold the encoded data
    let mut buffer = Vec::new();
    // Encode the timestamp as a 64-bit integer
    buffer.extend_from_slice(&timestamp.to_le_bytes());
    // Encode the temperature as 8 64-bit floats
    for &temp in &temperature {
        buffer.extend_from_slice(&temp.to_le_bytes());
    }
    // Encode the humidity as a 64-bit float
    buffer.extend_from_slice(&humidity.to_le_bytes());
    // Convert the buffer to a Box<[f64]>
    let boxed_slice: Box<[f64]> = buffer
        .chunks_exact(8)
        .map(|chunk| f64::from_le_bytes(chunk.try_into().unwrap()))
        .collect::<Vec<f64>>()
        .into_boxed_slice();
    // Return the boxed slice
    boxed_slice
}

// write test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let timestamp = 1234567890;
        let tempratures = [1.0, 2.0, 3.0, 2.2122, 2.2122, 2.2122];
        let humidity = 1.0;
        let result = encode(timestamp, tempratures, humidity);
        assert_eq!(result.len(), 8); // Adjust this based on the expected length of the output
        assert_eq!([6.09957582e-315, 1.0, 2.0, 3.0, 2.2122, 2.2122, 2.2122, 1.0], result.as_ref());
//        assert_eq!(result.as_ref()[0], 210f64);
//        assert_eq!(result.as_ref()[1..9], []);


    }
}
