use std::ops::*;

/**
 * File: Int.rs
 * 
 * Holder of the Int enum, to be used for integer representation with dynamic handling of the size of the integer.
 * 
 */
enum Int {
    I8Bit(i8),
    I16Bit(i16),
    I32Bit(i32),
    I64Bit(i64),
}

impl Int {
    pub fn init(value: i64) -> Int {
        match value {
            value if value <= i8::MAX as i64 && value >= i8::MIN as i64 => Int::I8Bit(value as i8),
            value if value <= i16::MAX as i64 && value >= i16::MIN as i64 => Int::I16Bit(value as i16),
            value if value <= i32::MAX as i64 && value >= i32::MIN as i64 => Int::I32Bit(value as i32),
            _ => Int::I64Bit(value),
        }
    }

    fn get_value(&self) -> i64 {
        match *self {
            Int::I8Bit(val) => val as i64,
            Int::I16Bit(val) => val as i64,
            Int::I32Bit(val) => val as i64,
            Int::I64Bit(val) => val,
        }
    }

    fn op<F>(&self, other: &Int, op: F) -> Int
    where
        F: Fn(i64, i64) -> i64,
    {
        let self_value: i64 = self.get_value();
        let other_value: i64 = other.get_value();

        let result: i64 = op(self_value, other_value);

        Int::init(result)
    }
}

impl Add for Int {
    type Output = Int;

    fn add(self, other: Int) -> Int {
        self.op(&other, |a: i64, b: i64| a + b)
    }
}

impl Sub for Int {
    type Output = Int;
    
    fn sub(self, other: Int) -> Int {
        self.op(&other, |a: i64, b: i64| a - b)
    }
}
