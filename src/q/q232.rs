struct MyQueue {
    s1: Vec<i32>,
    s2: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    /** Initialize your data structure here. */
    fn new() -> Self {
        MyQueue {
            s1: Vec::new(),
            s2: Vec::new(),
        }
    }

    /** Push element x to the back of queue. */
    fn push(&mut self, x: i32) {
        self.s1.push(x);
    }

    /** Removes the element from in front of queue and returns that element. */
    fn pop(&mut self) -> i32 {
        if self.s2.is_empty() {
            while let Some(x) = self.s1.pop() {
                self.s2.push(x);
            }
        }
        self.s2.pop().unwrap()
    }

    /** Get the front element. */
    fn peek(&mut self) -> i32 {
        if self.s2.is_empty() {
            while let Some(x) = self.s1.pop() {
                self.s2.push(x);
            }
        }
        *self.s2.last().unwrap()
    }

    /** Returns whether the queue is empty. */
    fn empty(&self) -> bool {
        self.s1.is_empty() && self.s2.is_empty()
    }
}