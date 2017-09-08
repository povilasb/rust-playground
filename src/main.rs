#[cfg(test)] #[macro_use] extern crate hamcrest;

struct Queue<T: Copy> {
    value: T,
    next: Option<Box<Queue<T>>>,
}

impl <T: Copy> Queue<T> {
    fn new(value: T) -> Queue<T> {
        Queue {
            value,
            next: None,
        }
    }

    fn append(&mut self, value: T) {
        append_node(self, Box::new(Queue::new(value)));
    }
}

fn append_node<T: Copy>(queue: &mut Queue<T>, node: Box<Queue<T>>) {
    if queue.next.is_some() {
        append_node(queue.next.as_mut().unwrap(), node);
    } else {
        queue.next = Some(node);
    }
}

struct QueueIterator<'a, T: Copy + 'a> {
    root_item: &'a Queue<T>,
    curr_item: &'a mut Queue<T>,
}

impl <'a, T: Copy> Iterator for QueueIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        /*
        match self.curr_item.next {
            Some(ref mut next_item) => self.curr_item = next_item,
            None => (),
        }
        */
        None
    }
}


fn main() {
    let mut q = Queue::new(1);
    q.append(2);
    q.append(3);

    let mut curr_item = q.next.as_ref().unwrap();
    curr_item = curr_item.next.as_ref().unwrap();
    println!("{}", curr_item.value);
}


#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    // Trying to emulate nesting what "stainless" lib already does.
    mod queue {
        use super::*;

        #[test]
        fn new_returns_queue_with_specified_item() {
            let q = Queue::new(10);

            assert_that!(q.value, is(equal_to(10)));
        }

        #[test]
        fn append_sets_next_queue_item() {
            let mut q = Queue::new(1);

            q.append(2);

            assert_that!(q.next.unwrap().value, is(equal_to(2)));
        }
    }
}
