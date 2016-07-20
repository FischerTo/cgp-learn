use std::fmt::Display;

struct Swagger<T: Display> {
    x: T,
}

impl<T: Display> Display for Swagger<T> {
    fn fmt(&self, form: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(form, "#swag {} #yolo", self.x)
    }
}


fn main() {
    let swag = Swagger { x: 456.6 };

    println!("{}", swag);


}
