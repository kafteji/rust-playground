#![allow(unused)]

use core::mem::MaybeUninit;

pub struct Queue<T, const SIZE: usize> {
    head: usize,
    tail: usize,
    size: usize,
    capacity: usize,
    buffer: [MaybeUninit<T>; SIZE],
}

impl<T, const SIZE: usize> Queue<T, SIZE> {
    fn new() -> Queue<T, SIZE> {
        assert!(SIZE > 0, "Queue size must be bigger than zero !");
        Self {
            head: 0,
            tail: 0,
            size: 0,
            capacity: SIZE,
            buffer: [const { MaybeUninit::uninit() }; SIZE],
        }
    }

    fn empty(&self) -> bool {
        self.size == 0
    }

    fn size(&self) -> usize {
        self.size
    }

    fn push(&mut self, elem: T) -> () {
        assert!(
            self.size + 1 <= self.capacity,
            "cannot push to queue because it is full."
        );
        self.buffer[self.head].write(elem);
        self.head = (self.head + 1) % self.capacity;
        self.size += 1;
    }

    fn pop(&mut self) -> T {
        assert!(self.size > 0, "cannot pop from queue because it is empty.");
        let elem: T = unsafe { self.buffer[self.tail].assume_init_read() };
        self.tail = (self.tail + 1) % self.capacity;
        self.size -= 1;
        elem
    }
}

#[cfg(test)]
mod queue_tests {
    use core::assert_eq;

    use crate::data_structures::queue::Queue;
    #[test]
    fn create_an_empty_queue_for_different_types() {
        let byte_queue: Queue<u8, 5> = Queue::new();
        assert!(byte_queue.empty());
        assert_eq!(byte_queue.size(), 0);

        let int_queue: Queue<u32, 42> = Queue::new();
        assert!(int_queue.empty());
        assert_eq!(int_queue.size(), 0);

        let string_queue: Queue<String, 8> = Queue::new();
        assert!(string_queue.empty());
        assert_eq!(string_queue.size(), 0);
    }

    #[test]
    #[should_panic]
    fn pop_an_empty_queue() {
        let mut q: Queue<u8, 8> = Queue::new();
        let result = q.pop();
    }

    #[test]
    fn push_and_pop_an_element() {
        let mut q: Queue<u8, 8> = Queue::new();
        let pushed_elem: u8 = 0xAA;
        q.push(pushed_elem);
        assert_eq!(q.size(), 1);
        let popped_elem = q.pop();
        assert_eq!(pushed_elem, popped_elem);
        assert!(q.empty());
    }

    #[test]
    fn push_and_pop_a_non_pod_element() {
        let mut q: Queue<String, 4> = Queue::new();
        let pushed_string = String::from("Hello");
        q.push(pushed_string);
        assert_eq!(q.size(), 1);
        let popped_string = q.pop();
        assert_eq!(popped_string, String::from("Hello"));
    }

    #[test]
    #[should_panic]
    fn push_an_element_to_a_full_queue() {
        let mut q: Queue<u8, 4> = Queue::new();
        for i in 0..4 {
            q.push(i);
        }
        assert_eq!(q.size(), 4);
        q.push(5);
    }

    #[test]
    fn push_pop_multiple_element_more_than_the_queue_capacity() {
        let mut q: Queue<u8, 4> = Queue::new();
        for i in 0..4 {
            q.push(i);
        }
        assert_eq!(q.size(), 4);
        let x = q.pop();
        assert_eq!(x, 0);
        assert_eq!(q.size(), 3);
        q.push(5);
        assert_eq!(q.size(), 4);
        assert_eq!(q.head, 1);
        assert_eq!(q.tail, 1);
    }
}
