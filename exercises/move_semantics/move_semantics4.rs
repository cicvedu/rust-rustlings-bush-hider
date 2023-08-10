// move_semantics4.rs
//
// Refactor this code so that instead of passing `vec0` into the `fill_vec`
// function, the Vector gets created in the function itself and passed back to
// the main function.
//
// Execute `rustlings hint move_semantics4` or use the `hint` watch subcommand
// for a hint.

<<<<<<< HEAD

fn main() {

    let mut vec1 = fill_vec();
=======
// I AM NOT DONE

fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);
>>>>>>> 24007d7b5f816a54e94386df6064a182a1cb66bc

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// `fill_vec()` no longer takes `vec: Vec<i32>` as argument
fn fill_vec() -> Vec<i32> {
<<<<<<< HEAD
    let mut vec = Vec::new();
=======
    let mut vec = vec;
>>>>>>> 24007d7b5f816a54e94386df6064a182a1cb66bc

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
