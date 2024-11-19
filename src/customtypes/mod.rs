use std::ops::*;

/**
 * File: Int.rs
 * 
 * Holder of the Int enum, to be used for integer representation with dynamic handling of the size of the integer.
 * 
 */
pub enum Int {
    I8Bit(i8),
    I16Bit(i16),
    I32Bit(i32),
    I64Bit(i64),
}

impl Int {
    /**
     * # init
     * 
     * Initializes the Int enum with the value provided.
     * 
     * ## Parameters
     * 
     * - `value`: An integer value that will be used to initialize the Int enum.
     * 
     * ## Returns
     * 
     * An Int enum with the value provided.
     * 
     */
    pub fn init(value: i64) -> Int {
        match value {
            value if value <= i8::MAX as i64 && value >= i8::MIN as i64 => Int::I8Bit(value as i8),
            value if value <= i16::MAX as i64 && value >= i16::MIN as i64 => Int::I16Bit(value as i16),
            value if value <= i32::MAX as i64 && value >= i32::MIN as i64 => Int::I32Bit(value as i32),
            _ => Int::I64Bit(value),
        }
    }

    /**
     * # get_value
     * 
     * Returns the value of the Int enum.
     * 
     * ## Returns
     * 
     * The value of the Int enum.
     * 
     */
    pub fn get_value(&self) -> i64 {
        match *self {
            Int::I8Bit(val) => val as i64,
            Int::I16Bit(val) => val as i64,
            Int::I32Bit(val) => val as i64,
            Int::I64Bit(val) => val,
        }
    }

    /**
     * # _get_type
     * 
     * Returns the type of the Int enum.
     * 
     * ## Returns
     * 
     * A string that represents the type of the Int enum.
     * 
     */
    pub fn _get_type(&self) -> String {
        match *self {
            Int::I8Bit(_) => "i8".to_string(),
            Int::I16Bit(_) => "i16".to_string(),
            Int::I32Bit(_) => "i32".to_string(),
            Int::I64Bit(_) => "i64".to_string(),
        }
    }

    /**
     * # op
     * 
     * Performs an operation on two Int enums.
     * 
     * ## Parameters
     * 
     * - `other`: A reference to an Int enum that will be used in the operation.
     * - `op`: A function that will be used to perform the operation.
     * 
     * ## Returns
     * 
     * An Int enum with the result of the operation.
     * 
     */
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

/**
 * Implementations of the Add, Sub, Mul, and Div traits for the Int enum.
 * 
 */
impl Mul for Int {
    type Output = Int;

    fn mul(self, other: Int) -> Int {
        self.op(&other, |a: i64, b: i64| a * b)
    }
}

impl Div for Int {
    type Output = Int;

    fn div(self, other: Int) -> Int {
        self.op(&other, |a: i64, b: i64| a / b)
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
