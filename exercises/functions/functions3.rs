// functions3.rs
// Make me compile! Execute `rustlings hint functions3` for hints :)

fn main() {
    let i: i32 = 12;
    call_me(i);
}

fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
