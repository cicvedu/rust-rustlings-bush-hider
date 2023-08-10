// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

<<<<<<< HEAD
// ## two writing reference : 同时存在两个可写引用（&mut variab）
=======
// I AM NOT DONE
>>>>>>> 24007d7b5f816a54e94386df6064a182a1cb66bc

fn main() {
    let mut x = 100;
    let y = &mut x;
<<<<<<< HEAD
    *y += 100;
    let z = &mut x;
=======
    let z = &mut x;
    *y += 100;
>>>>>>> 24007d7b5f816a54e94386df6064a182a1cb66bc
    *z += 1000;
    assert_eq!(x, 1200);
}
