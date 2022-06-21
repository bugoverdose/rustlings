// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` for hints :)

fn main() {
    let mut x = 100;
    let y = &mut x; 
    *y += 100; // y에 100을 더하고 x에 대한 참조 끊기(둘 다 200)
    let z = &mut x;
    *z += 1000; // z에 1000을 더하고 x에 대한 참조 끊기(둘 다 1200)
    assert_eq!(x, 1200);
}
// & : reference
// * : dereference