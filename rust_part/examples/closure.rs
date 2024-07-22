fn main() {
    let input = Some("42");

    let valid = input.map(|num| num.parse::<u32>().ok()).and_then(|num| {
        if num >= 0 && num <= 100 {
            Some(num)
        } else {
            None
        }
    });

    println!("{:?}", valid);
}
