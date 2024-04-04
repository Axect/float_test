use std::os::raw::{c_float, c_double, c_int};
use peroxide::fuga::*;

fn main() {
    let n = Normal(0.0, 1.0);
    let x_vec = n.sample(1000);

    x_vec.iter().all(|&x| {
        let (m1, e1) = x.frexp();
        let (m2, e2) = frexp_(x);
        m1 == m2 && e1 == e2
    }).print();

    let n = Uniform(0i32, 10i32);
    let i_vec = n.sample(1000);

    x_vec.iter().zip(i_vec.iter()).all(|(&x, &i)| {
        let m1 = x.ldexp(i as i32);
        let m2 = ldexp_(x, i as i32);
        m1 == m2
    }).print();
}

// ┌─────────────────────────────────────────────────────────┐
//  frexp & ldexp from C
// └─────────────────────────────────────────────────────────┘
extern "C" {
    fn frexp(x: c_double, exp: *mut c_int) -> c_double;
    fn frexpf(x: c_float, exp: *mut c_int) -> c_float;
    fn ldexp(x: c_double, exp: c_int) -> c_double;
    fn ldexpf(x: c_float, exp: c_int) -> c_float;
}

pub trait FloatExp: Sized {
    fn frexp(self) -> (Self, i32);
    fn ldexp(self, exp: i32) -> Self;
}

impl FloatExp for f64 {
    fn frexp(self) -> (Self, i32) {
        let mut exp: c_int = 0;
        let res = unsafe { frexp(self, &mut exp) };
        (res, exp)
    }

    fn ldexp(self, exp: i32) -> Self {
        unsafe { ldexp(self, exp) }
    }
}

impl FloatExp for f32 {
    fn frexp(self) -> (Self, i32) {
        let mut exp: c_int = 0;
        let res = unsafe { frexpf(self, &mut exp) };
        (res, exp)
    }

    fn ldexp(self, exp: i32) -> Self {
        unsafe { ldexpf(self, exp) }
    }
}

// ┌─────────────────────────────────────────────────────────┐
//  Pure Rust frexp
// └─────────────────────────────────────────────────────────┘
fn frexp_(x: f64) -> (f64, i32) {
    // If the input is zero, return (0.0, 0)
    if x == 0.0 {
        return (0.0, 0);
    }

    // Convert the input to its IEEE 754 binary representation
    let bits = x.to_bits();

    // Determine the sign of the input
    // If the most significant bit (bit 63) is 1, the input is negative
    let sign = if (bits >> 63) != 0 { -1.0 } else { 1.0 };

    // Extract the exponent from the binary representation
    // Bits 52 to 62 represent the exponent in IEEE 754 format
    // Subtract 1023 to obtain the actual exponent value
    let exponent = ((bits >> 52) & 0x7ff) as i32 - 1023;

    // Extract the mantissa (significand) from the binary representation
    // Bits 0 to 51 represent the mantissa
    // Set the implicit leading bit (bit 52) to 1 to normalize the mantissa
    // Multiply the mantissa by the sign to handle negative inputs correctly
    let mantissa = sign * f64::from_bits((bits & 0xfffffffffffff) | 0x3fe0000000000000);

    // Return the normalized mantissa and the exponent incremented by 1
    (mantissa, exponent + 1)
}

// ┌─────────────────────────────────────────────────────────┐
//  Pure Rust ldexp
// └─────────────────────────────────────────────────────────┘
fn ldexp_(x: f64, exp: i32) -> f64 {
    // If the input is zero or the exponent is zero, return the input unchanged
    if x == 0.0 || exp == 0 {
        return x;
    }

    // Convert the input to its IEEE 754 binary representation
    let bits = x.to_bits();

    // Extract the exponent from the binary representation
    // Bits 52 to 62 represent the exponent in IEEE 754 format
    let exponent = ((bits >> 52) & 0x7ff) as i32;

    // Calculate the new exponent by adding the input exponent to the existing exponent
    let new_exponent = exponent + exp;

    // Check if the new exponent is within the valid range for IEEE 754 format
    if !(0..=0x7ff).contains(&new_exponent) {
        // If the exponent is out of range, return infinity or zero depending on the input sign
        return if (bits >> 63) != 0 {
            f64::NEG_INFINITY
        } else {
            f64::INFINITY
        };
    }

    // Combine the new exponent with the mantissa and sign bits to create the result
    let result_bits = (bits & 0x800fffffffffffff) | ((new_exponent as u64) << 52);
    f64::from_bits(result_bits)
}
