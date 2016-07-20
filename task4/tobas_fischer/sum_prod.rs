use std::ops::{Add, Mul};

fn main() {
    println!("{}, {}", sf(2, 4).0, sf(2, 4).1)
}

pub fn sf<T: Add + Mul + Copy>(x: T, y: T) -> (<T as Add>::Output, <T as Mul>::Output) {
    (x + y, x * y)
}
