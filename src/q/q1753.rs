use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn maximum_score(a: i32, b: i32, c: i32) -> i32 {
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
