// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let mut x = 100;
    let y = &mut x;
<<<<<<< HEAD
    *y += 100;
    let z = &mut x;
    *z += 1000;
=======
   
    *y += 1100;

>>>>>>> 1f3cbd2539077ab6471e6235afeea21f46df7860
    assert_eq!(x, 1200);
}
