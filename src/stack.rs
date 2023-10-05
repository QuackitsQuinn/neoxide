use crate::reg::{Register, U8Register};

pub struct Stack {
    stack: [u8; 0x100],
    pub sp: U8Register,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            stack: [0; 0x100],
            sp: U8Register::new(0xFF),
        }
    }

    pub fn push(&mut self, data: u8) {
        self.stack[self.sp.read() as usize] = data;
        self.sp -= 1;
    }

    pub fn pop(&mut self) -> u8 {
        self.sp += 1;
        self.stack[self.sp.read() as usize]
    }
}

#[cfg(test)]
mod testStack {

    use super::*;

    #[test]
    fn test_push() {
        let mut stack = Stack::new();
        stack.push(0x12);
        assert_eq!(stack.stack[0xFF], 0x12);
        assert_eq!(stack.sp.read(), 0xFE);
    }

    #[test]
    fn test_pop() {
        let mut stack = Stack::new();
        stack.stack[0xFF] = 0x12;
        stack.sp.write(0xFE);
        assert_eq!(stack.pop(), 0x12);
        assert_eq!(stack.sp.read(), 0xFF);
    }
}
