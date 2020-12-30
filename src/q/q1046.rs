use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        // 方法1
        // 直接用大顶堆解决
        // Passed 0ms 2mb
        let mut heap = stones.into_iter().collect::<std::collections::BinaryHeap<i32>>();
        while heap.len() > 1 {
            let y = heap.pop().unwrap();
            let x = heap.pop().unwrap();
            if y != x { heap.push(y - x); }
        }
        if let Some(x) = heap.pop() {
            x
        } else {
            0
        }
    }
}