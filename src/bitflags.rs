pub struct BitFlags {
    num: u8,
}

impl BitFlags {
    pub fn new(num: u8) -> BitFlags {
        BitFlags { num }
    }

    pub fn get(&self, i: usize) -> bool {
        if i > 7 {
            return false;
        }

        return (self.num >> i & 1) == 1;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get() {
        let flags = BitFlags::new(0b00000000);

        assert_eq!(flags.get(0), false);
        assert_eq!(flags.get(1), false);
        assert_eq!(flags.get(2), false);
        assert_eq!(flags.get(3), false);
        assert_eq!(flags.get(4), false);
        assert_eq!(flags.get(5), false);
        assert_eq!(flags.get(6), false);
        assert_eq!(flags.get(7), false);

        let flags = BitFlags::new(0b00001011);

        assert_eq!(flags.get(0), true);
        assert_eq!(flags.get(1), true);
        assert_eq!(flags.get(2), false);
        assert_eq!(flags.get(3), true);
        assert_eq!(flags.get(4), false);
        assert_eq!(flags.get(5), false);
        assert_eq!(flags.get(6), false);
        assert_eq!(flags.get(7), false);

        let flags = BitFlags::new(0b11111111);

        assert_eq!(flags.get(0), true);
        assert_eq!(flags.get(1), true);
        assert_eq!(flags.get(2), true);
        assert_eq!(flags.get(3), true);
        assert_eq!(flags.get(4), true);
        assert_eq!(flags.get(5), true);
        assert_eq!(flags.get(6), true);
        assert_eq!(flags.get(7), true);

        assert_eq!(flags.get(8), false);
        assert_eq!(flags.get(9), false);
        assert_eq!(flags.get(10), false);
    }
}
