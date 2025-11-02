fn main() {
    // https://doc.rust-lang.org/reference/tokens.html#byte-string-literals
    // https://doc.rust-lang.org/reference/expressions/literal-expr.html#byte-literal-expressions
    let ch: u8 = b'A';      // byte literal, like C/C++ char
    let s: i16 = ch as i16; // cast to 16-bit signed
    println!("{}", s);      // prints 65
}

// fn main() {
//     let ch: char = 'A';            // Unicode scalar, 32-bit
//     let s: i16 = ch as u32 as i16; // code point → i16
//     println!("{}", s);             // prints 65
// }