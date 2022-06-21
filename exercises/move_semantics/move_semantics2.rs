// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` for hints :)

fn main() {
    main1();
    main2();
    main3();
}

// borrow
fn main1() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec1(&vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec1(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec.to_vec();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

// ====================

// clone
fn main2() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec2(vec0.clone());

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec2(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

// ====================

// mutably borrow : 참조를 통해 힙의 데이터 수정!
fn main3() {
    let mut vec0 = Vec::new();

    fill_vec3(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
}

fn fill_vec3(vec: &mut Vec<i32>) -> () {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}

// ====================

// BAD WAY!
// fn main() {
//     let vec0 = Vec::new();

//     let (vec0, mut vec1) = fill_vec(vec0);

//     // Do not change the following line!
//     println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

//     vec1.push(88);

//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }

// fn fill_vec(vec0: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
//     let mut vec1 = vec0.clone();

//     vec1.push(22);
//     vec1.push(44);
//     vec1.push(66);

//     (vec0, vec1)
// }