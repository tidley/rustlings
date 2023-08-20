// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age)/* your pattern here */ = cat;
    let x: (i32, f64, u8) = (500, 6.4, 1);
    println!("{} is {} years old.", name, age);
}
