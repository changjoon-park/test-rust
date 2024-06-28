struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }

    fn as_tuple(&self) -> (&T, &U) {
        (&self.first, &self.second)
    }
}

fn main() {
    // Create a Pair of integers
    let int_pair = Pair::new(1, 2);
    println!("Integer Pair: {:?}", int_pair.as_tuple());

    // Create a Pair of a string and a float
    let mixed_pair = Pair::new("hello", 3.14);
    println!("Mixed Pair: {:?}", mixed_pair.as_tuple());

    // Create a Pair of custom types
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 10, y: 20 };
    let pair_with_point = Pair::new(point, "Label");
    println!("Pair with Point: {:?}", pair_with_point.as_tuple());
}
