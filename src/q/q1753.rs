use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn maximum_score(a: i32, b: i32, c: i32) -> i32 {
        // 方法1
        // 始终取最大的两个数来处理
        // 直到出现2个0及以上
        // 利用大顶堆来获取顶上两个最大的，处理完后再放入堆
        let mut answer = 0;
        let mut heap = std::collections::BinaryHeap::new();
        heap.push(a);
        heap.push(b);
        heap.push(c);
        loop {
            let m1 = heap.pop().unwrap();
            let m2 = heap.pop().unwrap();
            if m1 == 0 || m2 == 0 {
                break;
            }
            heap.push(m1 - 1);
            heap.push(m2 - 1);
            answer += 1;
        }
        answer
    }
}
