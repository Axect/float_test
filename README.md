# Floating-Point Functions Test

This repository contains Rust code for testing the implementation of two floating-point functions: `frexp` and `ldexp`. The code compares the results of pure Rust implementations with the corresponding C functions.

## Functions

### frexp

The `frexp` function breaks a floating-point number into a normalized fraction and an integral power of 2. It is typically used to extract the mantissa and exponent of a floating-point number.

The pure Rust implementation of `frexp` is provided in the code, along with the corresponding C function imported via FFI (Foreign Function Interface).

### ldexp

The `ldexp` function multiplies a floating-point number by a power of 2. It is typically used to construct a floating-point number from a mantissa and an exponent.

The pure Rust implementation of `ldexp` is provided in the code, along with the corresponding C function imported via FFI.

## Testing

The code generates random floating-point numbers using the `peroxide` library and tests the equivalence of the pure Rust implementations and the C functions for both `frexp` and `ldexp`.

For `frexp`, the code generates 1000 random numbers from a standard normal distribution (mean = 0, standard deviation = 1) and compares the results of the pure Rust implementation and the C function.

For `ldexp`, the code generates 1000 random floating-point numbers from a standard normal distribution and 1000 random integers from a uniform distribution (range: 0 to 10). It then tests the equivalence of the pure Rust implementation and the C function by applying `ldexp` to each pair of floating-point number and integer.

## Dependencies

The code relies on the following dependencies:
- `std::os::raw`: Provides FFI types for interoperability with C.
- `peroxide`: A comprehensive numerical library for Rust. Here, we use it to generate random numbers.

## Usage

To run the tests, execute the `main` function in `src/main.rs`. The code will generate random numbers, apply the `frexp` and `ldexp` functions, and compare the results of the pure Rust implementations with the C functions.

## License

This code is provided under the [MIT License](LICENSE).

Feel free to use, modify, and distribute the code as needed.
