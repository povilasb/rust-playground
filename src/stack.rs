pub struct Stack<T> {
    items: Vec<T>,
}

impl <T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            items: Vec::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
}
