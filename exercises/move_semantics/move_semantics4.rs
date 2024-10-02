// move_semantics4.rs
//
// Refactor this code so that instead of passing `vec0` into the `fill_vec`
// function, the Vector gets created in the function itself and passed back to
// the main function.
//
// Execute `rustlings hint move_semantics4` or use the `hint` watch subcommand
// for a hint.


fn main() {
<<<<<<< HEAD
    //let vec0 = Vec::new();

    let mut vec1 = fill_vec();
=======
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);
>>>>>>> 1f3cbd2539077ab6471e6235afeea21f46df7860

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// `fill_vec()` no longer takes `vec: Vec<i32>` as argument
<<<<<<< HEAD
fn fill_vec() -> Vec<i32> {
    let mut vec = Vec::new();
=======
fn fill_vec(vec:Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
>>>>>>> 1f3cbd2539077ab6471e6235afeea21f46df7860

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
