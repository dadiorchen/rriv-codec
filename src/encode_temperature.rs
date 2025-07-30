use crate::bits::Bits;


// temperature range: -40.0 to 140.0 Fahrenheit
// convert: f(x) = x * 10 + 400 
// projected range: 0 to 1800
// bits: 11 bits
pub fn encode_temperature(temperature: f64) -> Bits {
    let adjusted_temp = (temperature * 10.0 + 400.0) as u16;
    let mut data = vec![0u8; 2];
    data[0] = (adjusted_temp >> 8) as u8; // high byte
    data[1] = (adjusted_temp & 0xFF) as u8; // low byte
    Bits::new(data.into_boxed_slice(), 16)
}

pub fn decode_temperature(Bits: Bits) -> f64 {
    if Bits.len != 16 {
        panic!("Invalid bit length for temperature decoding");
    }
    let high_byte = Bits.data[0] as u16;
    let low_byte = Bits.data[1] as u16;
    let adjusted_temp = (high_byte << 8) | low_byte; // combine bytes
    (adjusted_temp as f64 - 400.0) / 10.0 // reverse the conversion
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_temperature() {
        let temperature = 25.0;
        let encoded = encode_temperature(temperature);
        assert_eq!(encoded.len, 56); 
        let decoded = decode_temperature(encoded);
        assert_eq!(decoded, temperature);
    }
}
