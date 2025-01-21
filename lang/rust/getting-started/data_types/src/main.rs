fn main() {
    let x = 5;
    // x = 6; // error: cannot assign twice to immutable variable `x`
    let x = 5; // shadowing

    let mut y = 5; // mutable
    y = 6;

    let z: i32 = 7; // type annotation

    let guess: u32 = "42".parse().expect("Not a number!"); // type inference

    /** Integer types in rust
     * Length, Signed, Unsigned
     * 8-bit, i8, u8
     * 16-bit, i16, u16
     * 32-bit, i32, u32
     * 64-bit, i64, u64
     * 128-bit, i128, u128
     * arch, isize, usize
     * 
     * Size calculations (where n is the number of bits):
     * Signed 8-bits: -2^(n-1) to 2^(n-1) - 1 Equals: -128 to 127
     * Unsigned 8-bits: 0 to 2^n - 1 Equals: 0 to 255
     */

    /** Number literals
     * 
     * Decimal: 98_222
     * Hex: 0xff
     * Octal: 0o77
     * Binary: 0b1111_0000
     * Byte (u8 only): b'A'
     */

    /** Floating-Point Types
     * 
     * f32, f64
     */

    /** The Character Type 
     * 
     * char
     *
    */
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    /** Tuple Type */
    let tup1 = (250, 77);
    let two_fifty = tup1.0;
    let seventy_seven = tup1.1;

    let tup2: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup2;

    /** Array Type */
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let jan = months[0];

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];

    let spaces = [0; 5]; // [0, 0, 0, 0, 0]
}
