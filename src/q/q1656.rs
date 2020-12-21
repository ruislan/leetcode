struct OrderedStream {
    ptr: usize,
    values: Vec<String>,
}

#[allow(unused)]
impl OrderedStream {
    // 方法1
    // ：）按照描述直接编写代码就行了
    // 只需要注意下起始是从1开始，所以我们初始化的数据是n + 1
    // 这样避免一些转换问题
    fn new(n: i32) -> Self {
        OrderedStream {
            ptr: 1,
            values: vec!["".to_string(); n as usize + 1],
        }
    }

    fn insert(&mut self, id: i32, value: String) -> Vec<String> {
        let mut id = id as usize;
        self.values[id] = value;

        let mut answer = Vec::new();
        if id == self.ptr {
            while id < self.values.len() && !self.values[id].is_empty() {
                answer.push(self.values[id].clone());
                id += 1;
            }
            self.ptr = id;
        }
        answer
    }
}