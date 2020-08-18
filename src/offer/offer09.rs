struct CQueue {
    s1: Vec<i32>,
    s2: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CQueue {
    // 要求用两个栈实现队列
    // 栈的特点是先进后出，队列的特点是先进先出
    // 所以append_tail实际就是入队列，delete_head就是出队列
    // 假设我们的栈是s1和s2,
    // 注意，这里的vec是栈，所以可用的方法我认为只能有pop、push和last()

    // 方法1
    // 入队列的时候，我们入栈s1
    // 出队列的时候，我们将栈s1的数据pop出来到s2中，并返回s2.pop()，然后再将s2剩余的数pop()出来到s1中
    // 也即是入队列是o(1)，出队列是O(n)
    // Passed 132ms 4.2mb

    // 方法2
    // 我们出栈的时候是O(n)，现在我们调整为入栈为O(n)，即是入栈的时候，读取的时候就是O(1)
    // 这里其实没有质的变化，时间的长短应该依赖于测试的进出比例
    // Passed 104ms 4.4mb

    // 方法3
    // 结合方法1和方法2进行优化，
    // 方法1我们在每次出队列的时候都会将数据拷贝回去，其实完全没有必要
    // 删除的时候，只要s2里面还有就直接pop()，如果没有了，才将s1倒过来，再处理
    // 这里时间就不同了，假设我们一半加入一半删除，
    // 而除了第一次删除转换需要O(n)时间以外，其他操作不管进出都是O(1)
    // 假设我们一次加入一次删除，这样加入O(1)，删除O(2)，还是O(1)
    // 这样我们扩大到随机加入随机删除，实际就是O(1)和O(2)，最后还是O(1)
    // Passed 80ms 4.2mb
    // Rust真的很diao哈，速度太快了，其他语言都是200ms以上。
    fn new() -> Self {
        CQueue { s1: Vec::new(), s2: Vec::new() }
    }

    fn append_tail(&mut self, value: i32) {
        // m1
        // self.s1.push(value);

        // m2
        // while let Some(val) = self.s2.pop() {
        //     self.s1.push(val);
        // }
        // self.s2.push(value);
        // while let Some(val) = self.s1.pop() {
        //     self.s2.push(val);
        // }

        // m3
        self.s1.push(value);
    }

    fn delete_head(&mut self) -> i32 {
        // m1
        // while let Some(val) = self.s1.pop() {
        //     self.s2.push(val);
        // }
        // let ret = match self.s2.pop() {
        //     None => -1,
        //     Some(val) => val,
        // };
        // while let Some(val) = self.s2.pop() {
        //     self.s1.push(val);
        // }
        // ret

        // m2
        // self.s2.pop().unwrap_or(-1)

        // m3
        if self.s2.is_empty() {
            while let Some(val) = self.s1.pop() {
                self.s2.push(val);
            }
        }
        self.s2.pop().unwrap_or(-1)
    }
}

/**
 * Your CQueue object will be instantiated and called as such:
 * let obj = CQueue::new();
 * obj.append_tail(value);
 * let ret_2: i32 = obj.delete_head();
 */
#[test]
fn test() {
    let mut obj = CQueue::new();
    obj.append_tail(32);
    assert_eq!(32, obj.delete_head());
    assert_eq!(-1, obj.delete_head());

    obj.append_tail(3);
    assert_eq!(3, obj.delete_head());
    assert_eq!(-1, obj.delete_head());

    assert_eq!(-1, obj.delete_head());
    obj.append_tail(5);
    obj.append_tail(2);
    assert_eq!(5, obj.delete_head());
    assert_eq!(2, obj.delete_head());
}