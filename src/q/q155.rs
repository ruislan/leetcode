struct MinStack {
    arr: Vec<(i32, Option<i32>)>,
    min: Option<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    // 本题同offer30，先做的offer30，解法看offer30
    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack {
            arr: Vec::new(),
            min: None,
        }
    }

    fn push(&mut self, x: i32) {
        let min = self.min;
        if min.is_none() || x < min.unwrap() { self.min = Some(x); }
        self.arr.push((x, min));
    }

    fn pop(&mut self) {
        self.min = self.arr.pop().unwrap().1;
    }

    fn top(&self) -> i32 {
        self.arr.last().unwrap().0
    }

    fn get_min(&self) -> i32 {
        self.min.unwrap()
    }
}