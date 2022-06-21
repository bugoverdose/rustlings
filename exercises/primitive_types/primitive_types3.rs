// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!

fn main() {
    let a = ["element"; 1000];
    // let mut a = String::from("");
    // for _ in (0..10) {
    //     a.push_str("1234567890");
    // }
    println!("{}", a.len()); // len : String, array 등에 존재하는 메서드. (not tuple)

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
