use rand::prelude::*;

struct Solution {
    nums: Vec<i32>,
    origin: Vec<i32>,
    n: usize,
}

#[allow(unused)]
impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Solution { nums: nums.clone(), origin: nums.clone(), n: nums.len() }
    }

    fn reset(&mut self) -> Vec<i32> {
        self.nums = self.origin.clone();
        self.nums.clone()
    }

    // 方法1
    // 模拟洗牌，假设我们有一堆牌0..n
    // 我们可以随机抽取一张这个牌，然后把这张牌放到最后
    // 在这里，我们可以把这张牌与最后一张牌进行交换
    // 也即是说最后一张牌已经洗好，
    // 那么要洗的牌就是第一张到倒数第二张，也即是0..n-1
    // 我们重复上述操作，一直到最后一张牌
    // 即完成了洗牌操作
    // AC 20ms 5.5mb
    fn shuffle(&mut self) -> Vec<i32> {
        for i in (0..self.n).rev() {
            let j = rand::thread_rng().gen_range(0..i + 1);
            self.nums.swap(i, j);
        }
        self.nums.clone()
    }
}