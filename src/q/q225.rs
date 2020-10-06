struct MyStack {
    q: std::collections::VecDeque<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
// 方法1
// 插入的时候，生成一个新的队列，然后让新的值放在最前面即可
impl MyStack {
    /** Initialize your data structure here. */
    fn new() -> Self {
        MyStack {
            q: std::collections::VecDeque::new(),
        }
    }

    /** Push element x onto stack. */
    fn push(&mut self, x: i32) {
        let mut new_q = std::collections::VecDeque::new();
        new_q.push_back(x);
        while let Some(x) = self.q.pop_front() {
            new_q.push_back(x);
        }
        self.q = new_q;
    }

    /** Removes the element on top of the stack and returns that element. */
    fn pop(&mut self) -> i32 {
        self.q.pop_front().unwrap()
    }

    /** Get the top element. */
    fn top(&mut self) -> i32 {
        *self.q.front().unwrap()
    }

    /** Returns whether the stack is empty. */
    fn empty(&self) -> bool {
        self.q.is_empty()
    }
}