fn main() {
    println!("Hello, world!");
}

pub fn encode( timestamp: i64,
               values: &[f64],
               bits: &[u8]
             ) -> Box<[u8]>{
    let mut buffer = Vec::new();
    let mut timestamp = timestamp;
    //buffer.push(0xFF);
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
        assert_eq!(result.len(), 0); // Adjust this based on the expected length of the output

    }
}
