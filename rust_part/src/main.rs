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
    let int_pair = Pair::new(1, 2);

    println!("{:?}", int_pair.as_tuple());

    let mixed_pair = Pair::new("hello", 3.14);

    println!("{:?}", mixed_pair.as_tuple());
}
