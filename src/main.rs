#[cfg(test)] #[macro_use] extern crate hamcrest;

struct Queue<T> {
    item: T,
    next: Option<Box<Queue<T>>>,
}

impl <T> Queue<T> {
    fn new(item: T) -> Queue<T> {
        Queue {
            item,
            next: None,
        }
    }

    fn append(&mut self, item: T) {
        self.next = Some(Box::new(Queue::new(item)));
    }
}


fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    // Trying to emulate nesting what "stainless" lib does.
    mod queue {
        use super::*;

        #[test]
        fn new_returns_queue_with_specified_item() {
            let q = Queue::new(10);

            assert_that!(q.item, is(equal_to(10)));
        }

        #[test]
        fn append_sets_next_queue_item() {
            let mut q = Queue::new(1);

            q.append(2);

            assert_that!(q.next.unwrap().item, is(equal_to(2)));
        }
    }
}
