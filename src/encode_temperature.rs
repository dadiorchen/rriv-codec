use crate::bits::Bits;
use crate::utils::number_to_binary_array;
use crate::utils::getBits;


// temperature range: -40.0 to 140.0 Fahrenheit
// convert: f(x) = x * 10 + 400 
// projected range: 0 to 1800
// bits: 11 bits
pub fn encode_temperature(temperature: f64) -> Bits {
    let adjusted_temp = (temperature * 10.0 + 400.0) as u16;
    let result = getBits(adjusted_temp, 11);
    println!("Encoded temperature: {:?}", result.to_string());
    result
}

pub fn decode_temperature(bits: Bits) -> f64 {
    if bits.len() != 11 {
        panic!("Invalid bit length for temperature decoding");
    }
    let number = u16::from_str_radix(&bits.to_string(), 2)
        .expect("Failed to convert bits to number");
    let adjusted_temp = number as f64 - 400.0;
    let temperature = adjusted_temp / 10.0;
    println!("Decoded temperature: {}", temperature);
    temperature
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_temperature() {
        let temperature = 0.0;
        let encoded = encode_temperature(temperature);
        assert_eq!(encoded.len(), 11); 
        let decoded = decode_temperature(encoded);
        assert_eq!(decoded, temperature);
    }
}
