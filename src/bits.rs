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
}
