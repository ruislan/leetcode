use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        // 方法1
        // 没啥好说的，按照题目描述直接做就行了
        // Passed 12ms 2mb
        let mut answer = vec![first];
        for i in 0..encoded.len() {
            answer.push(answer[answer.len() - 1] ^ encoded[i]);
        }
        answer
    }
}