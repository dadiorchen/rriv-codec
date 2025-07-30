// simply save number as bit
pub struct Bits {
    pub data: Box<[u8]>,
}

impl Bits {
    pub fn new(data: Box<[u8]>) -> Self {
        Bits { data}
    }

    // to string representation
    pub fn to_string(&self) -> String {
        self.data.iter()
            // reverse 
            .map(|bit| format!("{:01}", bit))
            .collect::<Vec<String>>()
            .join("")
    }

    // get length in bits
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn sub(&self, start: usize, end: usize) -> Self {
        if start >= end || end > self.len() {
            panic!("Invalid range for sub");
        }
        let sub_data = &self.data[start..end];
        Bits::new(sub_data.to_vec().into_boxed_slice())
    }

    // convert bit to vector of bytes, 
    // use 0 to fill the last byte if necessary
    pub fn to_vec(&self) -> Vec<u8> {
        let mut vec = Vec::with_capacity((self.len() + 7) / 8);
        for i in (0..self.len()).step_by(8) {
            let mut byte = 0u8;
            for j in 0..8 {
                if i + j < self.len() {
                    byte |= (self.data[i + j] & 1) << (7 - j);
                }
            }
            vec.push(byte);
        }
        vec
    }

    // create Bits from a vector of bytes
    //
    pub fn from_vec(vec: Vec<u8>) -> Self {
        // create new empty vector, go through the vec and convert each byte to bits
        let mut bits = Vec::with_capacity(vec.len() * 8);
        for byte in vec {
            for i in (0..8).rev() {
                bits.push((byte >> i) & 1);
            }
        }
        Bits::new(bits.into_boxed_slice())
    }
}
