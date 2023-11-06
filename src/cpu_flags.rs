use std::fmt::Display;

/// CPU Status Flags
#[derive(Debug)]
pub struct CPUStatus {
    pub status: u8,
}
//TODO: look into bitflags crate
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
    /// Gets if the last operation caused a carry
    pub fn is_carry(&self) -> bool {
        self.get_bit(0b0000_0001)
    }
    /// Gets if the last operation caused a zero
    pub fn is_zero(&self) -> bool {
        self.get_bit(0b0000_0010)
    }
    /// Gets if the last operation caused an interrupt
    pub fn is_interrupt(&self) -> bool {
        self.get_bit(0b0000_0100)
    }
    // NES cpu doesnt have a decimal mode

    /// Gets if the last operation caused an overflow
    pub fn is_overflow(&self) -> bool {
        self.get_bit(0b0100_0000)
    }

    /// Gets if the last operation caused a negative
    pub fn is_negative(&self) -> bool {
        self.get_bit(0b1000_0000)
    }

    pub fn is_decimal(&self) -> bool {
        self.get_bit(0b0000_1000)
    }

    /// Sets the carry flag
    pub fn set_carry(&mut self, value: bool) {
        self.set_bit(0b0000_0001, value);
    }
    /// Sets the zero flag
    pub fn set_zero(&mut self, value: bool) {
        self.set_bit(0b0000_0010, value);
    }
    /// Sets the interrupt flag
    pub fn set_interrupt(&mut self, value: bool) {
        self.set_bit(0b0000_0100, value);
    }
    /// Sets the decimal flag
    pub fn set_overflow(&mut self, value: bool) {
        self.set_bit(0b0100_0000, value);
    }
    /// Sets the negative flag
    pub fn set_negative(&mut self, value: bool) {
        self.set_bit(0b1000_0000, value);
    }

    pub fn set_decimal(&mut self, value: bool) {
        self.set_bit(0b0000_1000, value);
    }
}

impl Display for CPUStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CPUStatus")
            .field("carry", &self.is_carry())
            .field("zero", &self.is_zero())
            .field("interrupt", &self.is_interrupt())
            .field("overflow", &self.is_overflow())
            .field("negative", &self.is_negative())
            .field("decimal", &self.is_decimal())
            .finish()
    }
}
