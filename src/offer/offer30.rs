use crate::offer::Solution;

struct MinStack {
    arr: Vec<(i32, Option<i32>)>,
    min: Option<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    // 本题同q155一样
    // 方法1
    // 因为要求所有的解都在O(1)，也即是说所有的操作都是常数时间
    // 我们push和pop都能做到常数时间，但是获取最小值如何是常数时间即要重点考虑的
    // 假设我们存储了最小值就好了，这是第一个想法，那么假设正好pop掉了这个最小值，第二个最小值怎么办
    // 那要是我们用一个数组来存最小值就好了，pop掉最小值，我们就能知道下一个，但是怎么保存啊，快速排序可是O(nlogn)的
    // 转换一下思想，如果我们pop的那个数据里面能够带有除了它的最小值（前一个最小值）就好了
    // 完全可以办到，谁说我们底层实现的数组就只能装一个数字了，而且我们是Rust，我们可以用元组
    // 这样假设我们可以存储当前的最小值min为None，然后当我们push时
    // 如果当前数字比最小值小或者最小值为None，那么当前最小值就是min，并且元组存储(n, min)
    // 例如: push -2,(-2, None) min:-2;push 0, (0, -2) min:-2; push -3 (-3, -2) min = -3
    // 这个时候min直接取第一个-3，如果要pop，这个时候我们pop的时候，min就变成了-2，再min = -2
    // 再top = 0，我们再pop，这个时候min 还是 -2
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

    fn min(&self) -> i32 {
        self.min.unwrap()
    }
}

#[test]
fn test() {
    let mut min_stack = MinStack::new();
    min_stack.push(-2);
    min_stack.push(0);
    min_stack.push(-3);
    assert_eq!(min_stack.min(), -3);
    min_stack.pop();
    assert_eq!(min_stack.top(), 0);
    assert_eq!(min_stack.min(), -2);
    min_stack.pop();
    min_stack.pop();

    min_stack.push(1);
    assert_eq!(min_stack.min(), 1);
    assert_eq!(min_stack.top(), 1);
    min_stack.pop();

    min_stack.push(1);
    min_stack.push(-1);
    assert_eq!(min_stack.min(), -1);
    assert_eq!(min_stack.top(), -1);
    min_stack.pop();
    assert_eq!(min_stack.min(), 1);
    assert_eq!(min_stack.top(), 1);
    min_stack.pop();

    min_stack.push(-1);
    min_stack.push(0);
    min_stack.push(1);
    assert_eq!(min_stack.min(), -1);
    assert_eq!(min_stack.top(), 1);
    min_stack.pop();
    assert_eq!(min_stack.min(), -1);
    assert_eq!(min_stack.top(), 0);
    min_stack.pop();
    assert_eq!(min_stack.min(), -1);
    assert_eq!(min_stack.top(), -1);
    min_stack.pop();

}