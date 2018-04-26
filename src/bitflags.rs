pub struct BitFlags {
    num: u8
}

impl BitFlags {
    pub fn new(num: u8) -> BitFlags {
        BitFlags{ num }
    }

    pub fn get(&self, i: usize) -> bool {
        if i > 7 {
            return false;
        }

        return (self.num >> i & 1) == 1
    }
}