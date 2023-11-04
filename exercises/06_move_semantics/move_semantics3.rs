// move_semantics3.rs
//
// Make me compile without adding new lines -- just changing existing lines! (no
// lines with multiple semicolons necessary!)
//
// Execute `rustlings hint move_semantics3` or use the `hint` watch subcommand
// for a hint.

#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(vec0);

    assert_eq!(vec1, vec![22, 44, 66, 88]);
    // NOTE: ここではvec0はmoveされているので、vec0は使えない
    // println!("{:?}", vec0);
    // q: 所有権を戻すにはどうすればいいのか？
    // a: 戻すことはできない。関数の引数として渡すことで、所有権を渡すことができる
}

fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);

    vec
}
