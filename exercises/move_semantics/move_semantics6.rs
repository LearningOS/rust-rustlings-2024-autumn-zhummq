// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.


fn main() {
<<<<<<< HEAD
    let data = "Rust is great!".to_string();

    get_char(data.clone());

    string_uppercase(data);
=======
    let mut data = "Rust is great!".to_string();

    get_char(data.clone());

    string_uppercase(&mut data);
>>>>>>> 1f3cbd2539077ab6471e6235afeea21f46df7860
}

// Should not take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
<<<<<<< HEAD
fn string_uppercase(mut data: String) {
    data.to_uppercase();

    println!("{}", data);
=======
fn string_uppercase(data: &mut String) {
    *data = data.to_uppercase();

    println!("{}", *data);
>>>>>>> 1f3cbd2539077ab6471e6235afeea21f46df7860
}
