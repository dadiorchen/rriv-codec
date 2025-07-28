extern crate chrono;
use std::mem;
mod hex_dump;
use chrono::{DateTime, Utc, TimeZone};

fn main() {
}


struct Bits {
    data: Box<[u8]>,
    len: u8,
}

fn concat_bits(bits: &[Bits]) -> Bits {
    // Calculate the total length of the concatenated bits
    let total_len = bits.iter().map(|b| b.data.len()).sum::<usize>();
    let mut concatenated_data = Vec::with_capacity(total_len);
    
    // Concatenate the data from each Bits struct
    for bit in bits {
        concatenated_data.extend_from_slice(&bit.data);
    }
    
    // Create a Bits struct with the concatenated data and a len
    Bits {
        data: concatenated_data.into_boxed_slice(),
        len: 0, // Length is not used in this example, but can be set if needed
    }
}

fn encode_timestamp(timestamp: i64) -> Bits {
    // Unix timestamp for 2025-01-01T00:00:00Z
    let unix_of_2025 = 1735689600;  
    // Calculate the offset from the Unix timestamp
    let offset = timestamp - unix_of_2025;
    // cut to i32 
    let offset = offset as i32;
    // shift the offset to fit in 30 bits
    let offset_shifted = offset << 2; // Shift left by 2 bits to make space for the sign bit
    println!("Offset from 2025: {}", offset);
    println!("Offset shifted: {}", offset_shifted);
    // convert to bits
    println!("hex dump: {:X?}", offset_shifted.to_le_bytes());
    hex_dump::hex_dump(&offset_shifted.to_le_bytes());
    Bits {
        data: Box::new(offset_shifted.to_le_bytes()),
        len: 30, // Length of the bits
    }
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

/*
 * |------30------|--------------|----------------|----------------|
 *     timestamp
 *     offset of 
 *     2025 year
 *
 * */
pub fn encode_v1(
    timestamp: i64,
    // The temprature is a 8 element array, every one is 64-bit float
    temperature: [f64; 6],
    // The humidity is a 64-bit float
    humidity: i8,
) -> Box<[u8]> {

    let year_bits = encode_timestamp(timestamp);

    let bits = concat_bits(&[
        year_bits,
        Bits {
            data: Box::new([0; 8]),
            len: 0,
        },
        Bits {
            data: Box::new(timestamp.to_le_bytes()),
            len: 0,
        },
    ]);

    let result = bits.data;
    result
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
    let _humidity = data[7];

    // convert humidity to i8
    let humidity = _humidity as i8;
    
    DecodedData {
        timestamp,
        temperature,
        humidity,
    }
}

pub fn encode_naive( timestamp: i64, 
               values: &[f32]
             ) -> Box<[u8]>{
    
    println!("encode {} {}", timestamp, values[0]);
    // rprintln!("{} values", values.len());
    let mut bytes: [u8; 22] = [0; 22];
    // let timestamp_bytes = timestamp.to_le_bytes();
    // rprintln!("{:X?}", timestamp_bytes);
    let timestamp_bytes = timestamp.to_be_bytes();
    println!("{:X?}", timestamp_bytes);
    bytes[0..8].copy_from_slice(&timestamp_bytes);
    for i in 0..values.len(){
      let value = (values[i] * 100.0) as u32;
      let value_bytes = value.to_be_bytes();
      println!("{:?}", (i * 4 + 8)..(i * 4 + 12));
      bytes[(i * 4 + 8)..(i * 4 + 12)].copy_from_slice(&value_bytes);
      println!("{:X?}", value_bytes);
      if i == 2 { break }; // send up to 3 values
    }
    
    println!("{:X?}", bytes);
    return Box::new(bytes);
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

        let total = result.len() * mem::size_of::<f64>();
        println!("size of f64: {}", mem::size_of::<f64>());
        println!("length of result: {}", result.len());
        println!("Total size: {}", total);
        assert_eq!(total, 64);
        // convert to hex
        let result_u8 = result
            .iter()
            .flat_map(|&x| x.to_le_bytes())
            .collect::<Vec<u8>>();
        hex_dump::hex_dump(&result_u8);


        // decode the result, the result is a struct with timestamp, temperature and humidity
        let result_decoded = decode(&result);
        assert_eq!(result_decoded.timestamp, timestamp_i64);
        assert_eq!(result_decoded.temperature, temperatures);
        assert_eq!(result_decoded.humidity, humidity);


    }

    #[test]
    fn test_encode_timestamp() {
        let time_str = "2025-07-28T00:00:00+00:00";
        let datetime = DateTime::parse_from_rfc3339(time_str).expect("Failed to parse date");
        // use i64 to store the timestamp
        let timestamp_i64 = datetime.timestamp();
        let bits = encode_timestamp(timestamp_i64);
        println!("Encoded bits: {:?}", bits.data);
        assert_eq!(bits.data.len(), 4); // 30 bits can fit in 4 bytes
        assert_eq!(bits.len, 30); // Length of the bits
        assert_eq!(bits.data.as_ref(), &[0, 0xe0, 0x48, 0x04]); // Adjust this based on the expected
    }

    #[test]
    fn test_encode_v1() {
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
        let result = encode_v1(timestamp_i64, temperatures, humidity);
        println!("Encoded data size: {}", result.len());
        println!("Encoded data use space: {} bytes", mem::size_of_val(&result));
        hex_dump::hex_dump(&result);
        //assert_eq!(result.len(), 8); // Adjust this based on the expected length of the output
        assert_eq!([0, 0, 0, 0, 0x65, 0x19, 0x5F, 0x40, 0, 0, 0x9, 0x88, 0, 0, 0x9, 0x92, 0, 0, 9, 0xBA, 0, 0], result.as_ref());


    }

    #[test]
    fn test_encode_naive() {
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
        let result = encode_naive(timestamp_i64, temperatures.as_ref());
        println!("Encoded data size: {}", result.len());
        println!("Encoded data use space: {} bytes", mem::size_of_val(&result));
        assert_eq!(result.len(), 22); 
        println!("{:X?}", result);
        assert_eq!([0, 0, 0, 0, 0x65, 0x19, 0x5F, 0x40, 0, 0, 0x9, 0x88, 0, 0, 0x9, 0x92, 0, 0, 9, 0xBA, 0, 0], result.as_ref());
        //assert_eq!([8.380151764e-315, 24.4, 24.5, 24.9, 25.9, 28.1, 30.2, 10.0], result.as_ref());
        assert_eq!(mem::size_of_val(&result), 16); 
        let total = result.len() * mem::size_of::<u8>();
        println!("Total size: {}", total);
        assert_eq!(total, 22);
        hex_dump::hex_dump(&result);

        // decode the result, the result is a struct with timestamp, temperature and humidity
//        let result_decoded = decode(&result);
//        assert_eq!(result_decoded.timestamp, timestamp_i64);
//        assert_eq!(result_decoded.temperature, temperatures);
//        assert_eq!(result_decoded.humidity, humidity);


    }
}
