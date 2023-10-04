
/// CPU Status Flags
pub struct CPUStatus {
    pub status: u8
}
//TODO: good doc comments
impl CPUStatus {
    /// Create a new CPUStatus
    pub fn new() -> Self {
        CPUStatus { status: 0 }
    }
    /// Gets the value of a bit in the status
    fn get_bit(&self, bit: u8) -> bool {
        self.status & bit == bit
    }
    /// Sets the value of a bit in the status
    fn set_bit(&mut self, bit: u8, value: bool) {
        if value {
            self.status |= bit;
        } else {
            self.status &= !bit;
        }
    }

    pub fn is_carry(&self) -> bool {
        self.get_bit(0b0000_0001)
    }

    pub fn is_zero(&self) -> bool {
        self.get_bit(0b0000_0010)
    }

    pub fn is_interrupt(&self) -> bool {
        self.get_bit(0b0000_0100)
    }
    // NES cpu doesnt have a decimal mode
    pub fn is_overflow(&self) -> bool {
        self.get_bit(0b0100_0000)
    }

    pub fn is_negative(&self) -> bool {
        self.get_bit(0b1000_0000)
    }

    pub fn set_carry(&mut self, value: bool) {
        self.set_bit(0b0000_0001, value);
    }

    pub fn set_zero(&mut self, value: bool) {
        self.set_bit(0b0000_0010, value);
    }

    pub fn set_interrupt(&mut self, value: bool) {
        self.set_bit(0b0000_0100, value);
    }

    pub fn set_overflow(&mut self, value: bool) {
        self.set_bit(0b0100_0000, value);
    }

    pub fn set_negative(&mut self, value: bool) {
        self.set_bit(0b1000_0000, value);
    }
}
    