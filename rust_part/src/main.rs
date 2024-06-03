fn take_ownership(s1: String) {
    // s1 takes ownership of the String
    println!("{}", s1);
    // s1 is dropped here
}

fn main() {
    let s = String::from("Hello");
    take_ownership(s);
    // s is no longer valid here
}
