struct MinStack {
    stack: Vec<(i32, i32)>,
}

// 方法1
// 让stack存储两个元素组成的元组item
// item.0代表当前值，item.1代表当前值的情况下的最小值
// 这样top就是取item.0，get_min就是item.1
#[allow(unused)]
impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        let mut min = x;
        if let Some(top) = self.stack.last() {
            min = min.min(top.1);
        }
        self.stack.push((x, min));
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }

    fn get_min(&self) -> i32 {
        self.stack.last().unwrap().1
    }
}