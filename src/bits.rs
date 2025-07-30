pub struct Bits {
    pub data: Box<[u8]>,
    pub len: u8,
}

impl Bits {
    pub fn new(data: Box<[u8]>, len: u8) -> Self {
        Bits { data, len }
    }

    // to string representation
    pub fn to_string(&self) -> String {
        self.data.iter()
            // reverse 
            .rev()
            .map(|byte| format!("{:08b}", byte))
            .collect::<Vec<String>>()
            .join(" ")
    }
}
