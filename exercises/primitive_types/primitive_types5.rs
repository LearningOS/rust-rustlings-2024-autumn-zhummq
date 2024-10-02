// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.


<<<<<<< HEAD
fn main() {
    let cat = ("Furry McFurson", 3.5);
    let /* your pattern here */(name, age) = cat;
=======

fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name ,age) = cat;
>>>>>>> 1f3cbd2539077ab6471e6235afeea21f46df7860

    println!("{} is {} years old.", name, age);
}
