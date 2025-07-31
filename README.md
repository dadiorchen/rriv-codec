
# Codec for RRIV

## Specification of Codec v1

#### Data to encode:

    - 1 timestamp
    - 6 temperature values
    - 1 humidity value

#### Encode Specification:

- Total length required: 13 bytes (104 bits)

- Timestamp:

  - size: 30 bits
  
  - capacity: capable of representing time: 2025-01-01T00:00:00Z to 2055-01-01T00:00:00Z
  
  - precision: 1 second

- Temperature values:

  - size: 6 * 11 bits = 66 bits

  - capacity: capable of representing temperature values from -40.0 to 140.0 Fahrenheit

  - precision: 0.1 Fahrenheit

- Humidity value:

  - size: 8 bits
    
  - capacity: capable of representing humidity values from 0 to 100 percent
    
  - precision: 1 percent


## How to use 

Please check test here: 

```rust

    #[test]
    fn test_encode_v1() {
        let time_str = "2025-07-28T00:00:00+00:00";
        let datetime = DateTime::parse_from_rfc3339(time_str).expect("Failed to parse date");
        // use i64 to store the timestamp
        let timestamp_i64 = datetime.timestamp();
        // create date from timestamp
        let datetime_parsed = DateTime::<Utc>::from_timestamp(timestamp_i64, 0)
            .expect("Failed to create date from timestamp");
        let temperatures = [24.4, 24.5, 24.9, 25.9, 28.1, 30.2];
        let humidity = 10;

        let result = encode_v1(timestamp_i64, temperatures, humidity);
        println!("Encoded data size: {}", result.len());
        assert_eq!(result.len(), 13); 
        println!("Encoded data use space: {} bytes", mem::size_of_val(&result));
        assert_eq!(mem::size_of_val(&result), 16);
        hex_dump(&result);

        // decode 
        let decoded = decode_v1(&result);
        assert_eq!(decoded.timestamp, timestamp_i64);
        assert_eq!(decoded.temperatures, temperatures);
        assert_eq!(decoded.humidity, humidity);

    }
```


## How to run test

```
cargo test 
```
