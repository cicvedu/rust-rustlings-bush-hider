// move_semantics3.rs
//
// Make me compile without adding new lines-- just changing existing lines! (no
// lines with multiple semicolons necessary!)
//
// Execute `rustlings hint move_semantics3` or use the `hint` watch subcommand
// for a hint.

<<<<<<< HEAD
=======
// I AM NOT DONE
>>>>>>> 24007d7b5f816a54e94386df6064a182a1cb66bc

fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

<<<<<<< HEAD
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
=======
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
>>>>>>> 24007d7b5f816a54e94386df6064a182a1cb66bc
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
