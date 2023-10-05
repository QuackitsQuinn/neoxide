use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemorySegment {
    pub byte: u8,
}

impl MemorySegment {
    pub fn new(byte: u8) -> Self {
        MemorySegment { byte }
    }

    pub fn reset(&mut self) {
        self.byte = 0;
    }
}

impl ops::BitAnd<u8> for MemorySegment {
    type Output = u8;

    fn bitand(self, rhs: u8) -> Self::Output {
        self.byte & rhs
    }
}

impl ops::BitAnd<MemorySegment> for MemorySegment {
    type Output = MemorySegment;

    fn bitand(self, rhs: MemorySegment) -> Self::Output {
        MemorySegment {
            byte: self & rhs.byte,
        }
    }
}

impl ops::BitOr<u8> for MemorySegment {
    type Output = u8;

    fn bitor(self, rhs: u8) -> Self::Output {
        self.byte | rhs
    }
}

impl ops::BitOr<MemorySegment> for MemorySegment {
    type Output = MemorySegment;

    fn bitor(self, rhs: MemorySegment) -> Self::Output {
        MemorySegment {
            byte: self | rhs.byte,
        }
    }
}

impl ops::Not for MemorySegment {
    type Output = MemorySegment;

    fn not(self) -> Self::Output {
        MemorySegment { byte: !self.byte }
    }
}

impl ops::BitXor<u8> for MemorySegment {
    type Output = u8;

    fn bitxor(self, rhs: u8) -> Self::Output {
        self.byte ^ rhs
    }
}

impl ops::BitXor<MemorySegment> for u8 {
    type Output = MemorySegment;

    fn bitxor(self, rhs: MemorySegment) -> Self::Output {
        MemorySegment {
            byte: self ^ rhs.byte,
        }
    }
}

#[cfg(test)]
mod test_memsegment {
    use super::*;
    /// shorthand for MemorySegment::new
    fn new(val: u8) -> MemorySegment {
        MemorySegment::new(val)
    }

    #[test]
    fn test_bitand() {
        assert_eq!(new(0b1010) & 0b1111, 0b1010);
        assert_eq!(new(0b1010) & new(0b1111), new(0b1010));
    }

    #[test]
    fn test_bitxor() {
        assert_eq!(new(0b1010) ^ 0b1111, 0b0101);
        assert_eq!(0b1010 ^ new(0b1111), new(0b0101));
    }
    #[test]
    fn test_bitor() {
        assert_eq!(new(0b1010) | 0b1111, 0b1111);
        assert_eq!(new(0b1100) | new(0b0011), new(0b1111));
    }

    #[test]
    fn test_not() {
        assert_eq!(!new(0), new(255));
    }
}
