fn main() {

    println!("{}, {}", sf(2, 4).0, sf(2, 4).1)
}

pub fn sf<T: std::ops::Add + std::ops::Mul + Copy>
    (x: T,
     y: T)
     -> (<T as std::ops::Add>::Output, <T as std::ops::Mul>::Output) {
    (x + y, x * y)
}
