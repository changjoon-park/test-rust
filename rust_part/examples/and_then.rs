fn main() {
    let input = Some("42");

    let valid_number = input
        .map(|num| num.parse::<u32>().ok())
        .flatten()
        .map(|num| {
            if num >= 0 && num <= 100 {
                Some(num)
            } else {
                None
            }
        })
        .flatten();

    let valid_number = input
        .and_then(|num| num.parse::<u32>().ok())
        .and_then(|num| {
            if num >= 0 && num <= 100 {
                Some(num)
            } else {
                None
            }
        });
}
