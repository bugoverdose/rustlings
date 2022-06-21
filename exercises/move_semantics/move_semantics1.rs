// move_semantics1.rs
// Make me compile! Execute `rustlings hint move_semantics1` for hints :)

fn main() {
    let vec0 = Vec::new();

    // fill_vec 내부에서 mutable했던 vec을 반환받았지만
    // 현재 스코프의 vec1에서 새롭게 mutable하도록 명시 필요!
    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec; // 전달받은 인자를 mutable하게 변경

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
