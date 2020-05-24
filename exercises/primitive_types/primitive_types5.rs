// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` for hints!

fn main() {
    let cat = ("Furry McFurson", 3.5);
    // let (name, age) = cat;
    // to add type hints, provide a type for the entire tuple
    // oh does that imply that name and age are runtime or compile time tuple members?
    // for cat a tuple type is created (anonymous?)
    let (name, age) : (&str, f32) = cat;

    println!("{} is {} years old.", name, age);
}
