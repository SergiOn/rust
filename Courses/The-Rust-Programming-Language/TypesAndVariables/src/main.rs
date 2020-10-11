use std::mem;

fn main() {
    println!("Hello, world!");

    // let - immutable by default

    // unsigned 0 +, 0.. 255
    let a: u8 = 123; // 8 bits
    // a = 456;

    // signed 0 + -, -127.. 128
    let a_i: i8 = -123; // 8 bits

    println!("a = {}", a);
    println!("a_i = {}", a_i);

    // let mut - mutable

    let mut b: i8 = 0;
    println!("b = {}", b);

    b = 42;
    println!("b = {}", b);

    let mut c = 123456789; // 32-bit signed
    println!("c = {}, sizes = {} bytes", c, mem::size_of_val(&c));

    c = -1;
    println!("c = {}, sizes = {} bytes", c, mem::size_of_val(&c));

    // i8 u8 i16 u16 i32 u32 i64 u64

    // isize, usize
    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up = {} bytes, {}-bit OS", c, size_of_z, size_of_z * 8);

    // let d: char = 'x';
    let d = 'x';
    println!("d = {}, sizes = {} bytes", d, mem::size_of_val(&d));

    let e = 2.5; // double-precision, 8 bytes or 64 bits, f64
    // let e: f64 = 2.5;
    // let e: f32 = 2.5;
    println!("e = {}, sizes = {} bytes", e, mem::size_of_val(&e));

    // true false
    let g = false;
    println!("g = {}, sizes = {} bytes", g, mem::size_of_val(&g));

}
