use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        // 方法1
        // bfs，从0000开始，每次每个位置从有2个变化，一共是8个变化
        // 找到target的话，就返回，如果0000找到了9999都还没有，那就返回-1
        // AC 112ms 3.1mb
        // P.S. clone了很多数组，增加了时间，实际上可以转化成数字来处理，
        // 这样就不用clone数组了，用时应该会少很多
        use std::collections::{VecDeque, HashSet};
        let deadends = deadends.into_iter()
            .map(|x| x.bytes().map(|b| (b - b'0') as i32).collect::<Vec<i32>>())
            .collect::<HashSet<Vec<i32>>>();
        let target = target.bytes().map(|b| (b - b'0') as i32).collect::<Vec<i32>>();

        let start = vec![0, 0, 0, 0];
        if target == start { return 0; }
        if deadends.contains(&start) { return -1; }

        let mut seen = HashSet::new();
        seen.insert(start.clone());

        let mut answer = 1;
        let mut deque = VecDeque::new();
        deque.push_back(start);
        while !deque.is_empty() {
            let size = deque.len();
            for _ in 0..size {
                let mut curr = deque.pop_front().unwrap();
                // up
                for i in 0..4 {
                    let mut lock = curr.clone();
                    if lock[i] == 9 { lock[i] = 0; } else { lock[i] += 1; }
                    if lock == target { return answer; }
                    if deadends.contains(&lock) || seen.contains(&lock) { continue; }
                    deque.push_back(lock.clone());
                    seen.insert(lock);
                }
                // down
                for i in 0..4 {
                    let mut lock = curr.clone();
                    if lock[i] == 0 { lock[i] = 9; } else { lock[i] -= 1; }
                    if lock == target { return answer; }
                    if deadends.contains(&lock) || seen.contains(&lock) { continue; }
                    deque.push_back(lock.clone());
                    seen.insert(lock.clone());
                }
            }
            answer += 1;
        }
        -1
    }
}