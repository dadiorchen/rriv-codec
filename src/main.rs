extern crate chrono;
use std::mem;
use chrono::prelude::*;

fn main() {
}

pub fn encode( 
    timestamp: i64,
    // The temprature is a 8 element array, every one is 64-bit float
    temperature: [f64; 6],
    // The humidity is a 64-bit float
    humidity: i8,
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
    buffer.extend_from_slice(&(humidity as f64).to_le_bytes());
    // Convert the buffer to a Box<[f64]>
    let boxed_slice: Box<[f64]> = buffer
        .chunks_exact(8)
        .map(|chunk| f64::from_le_bytes(chunk.try_into().unwrap()))
        .collect::<Vec<f64>>()
        .into_boxed_slice();
    // print size of box 
    println!("Encoded data size: {}", boxed_slice.len());
    println!("Encoded data use space: {} bytes", mem::size_of_val(&boxed_slice));
    // Return the boxed slice
    boxed_slice
}

// DecodedData struct to hold the decoded data
#[derive(Debug, PartialEq)]
pub struct DecodedData {
    pub timestamp: i64,
    pub temperature: [f64; 6],
    pub humidity: i8,
}

pub fn decode(data: &[f64]) -> DecodedData {
    // Ensure the data has the expected length
    assert!(data.len() >= 8, "Data must contain at least 8 elements");
    
    // Decode the timestamp from the first 8 bytes
    let timestamp = i64::from_le_bytes
        (data[0].to_bits().to_le_bytes());
    
    // Decode the temperature from the next 6 elements
    let temperature = [
        data[1], data[2], data[3], data[4], data[5], data[6]
    ];
    
    // Decode the humidity from the last element
    let humidity = data[7];

    // convert humidity to i8
    let humidity = humidity as i8;
    
    DecodedData {
        timestamp,
        temperature,
        humidity,
    }
}

// write test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let time_str = "2023-10-01T12:00:00+00:00";
        let datetime = DateTime::parse_from_rfc3339(time_str).expect("Failed to parse date");
        // use i64 to store the timestamp
        let timestamp_i64 = datetime.timestamp();
        // create date from timestamp
        let datetime_parsed = DateTime::<Utc>::from_timestamp(timestamp_i64, 0)
            .expect("Failed to create date from timestamp");
        println!("Parsed datetime: {}", datetime);
        println!("Unix timestamp: {}", datetime.timestamp());
        println!("Parsed timestamp: {}", datetime_parsed.timestamp());
        println!("Parsed timestamp in text: {}", datetime_parsed.to_rfc3339());
        let temperatures = [24.4, 24.5, 24.9, 25.9, 28.1, 30.2];
        let humidity = 10;
        let result = encode(timestamp_i64, temperatures, humidity);
        println!("Encoded data size: {}", result.len());
        println!("Encoded data use space: {} bytes", mem::size_of_val(&result));
        assert_eq!(result.len(), 8); // Adjust this based on the expected length of the output
        assert_eq!([8.380151764e-315, 24.4, 24.5, 24.9, 25.9, 28.1, 30.2, 10.0], result.as_ref());
        assert_eq!(mem::size_of_val(&result), 16); 

        // decode the result, the result is a struct with timestamp, temperature and humidity
        let result_decoded = decode(&result);
        assert_eq!(result_decoded.timestamp, timestamp_i64);
        assert_eq!(result_decoded.temperature, temperatures);
        assert_eq!(result_decoded.humidity, humidity);


    }
}
