struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    fn size(&self) -> usize {
        self.elements.len()
    }
}

fn main() {
    // Create a stack of integers
    let mut int_stack = Stack::new();
    int_stack.push(1);
    int_stack.push(2);
    int_stack.push(3);

    println!("Integer stack size: {}", int_stack.size());
    println!("Popped from integer stack: {:?}", int_stack.pop());
    println!("Top of integer stack: {:?}", int_stack.peek());

    // Create a stack of strings
    let mut string_stack = Stack::new();
    string_stack.push("Hello".to_string());
    string_stack.push("World".to_string());

    println!("String stack size: {}", string_stack.size());
    println!("Popped from string stack: {:?}", string_stack.pop());
    println!("Top of string stack: {:?}", string_stack.peek());

    // Create a stack of custom structs
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let mut point_stack = Stack::new();
    point_stack.push(Point { x: 1, y: 2 });
    point_stack.push(Point { x: 3, y: 4 });

    println!("Point stack size: {}", point_stack.size());
    println!("Popped from point stack: {:?}", point_stack.pop());
    println!("Top of point stack: {:?}", point_stack.peek());
}
