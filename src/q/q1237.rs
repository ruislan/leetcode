/*
 * // This is the custom function interface.
 * // You should not implement it, or speculate about its implementation
 * struct CustomFunction;
 * impl CustomFunction {
 *    pub fn f(x:i32,y:i32)->i32{}
 * }
 */
use crate::q::Solution;

pub struct CustomFunction;

impl Solution {
    pub fn find_solution(customfunction: &CustomFunction, z: i32) -> Vec<Vec<i32>> {
        // 方法1
        // 暴力法
        // 让x和y都从1到1000,穷举每一对，看结果
        // Passed 80ms 2.1mb O(n^2)
        // let mut answer = Vec::new();
        // for x in 1..=1000 {
        //     for y in 1..=1000 {
        //         if z == customfunction.f(x, y) {
        //             answer.push(vec![x, y]);
        //         }
        //     }
        // }
        // answer

        // 方法2
        // 利用两个函数严格递增的特点，x和y对向移动
        // 当函数结果比z大的话，那么我们就把大的那个移动小
        // 当函数结果比z小的话，那么我们就把小的那个移动大
        // 当函数结果与z相等的话，就直接存储x,y
        // Passed 0ms 2.1mb
        // let (mut x, mut y) = (1, 1000);
        // let mut answer = Vec::new();
        // while x < 1001 && y > 0 {
        //     match customfunction.f(x, y).cmp(&z) {
        //         std::cmp::Ordering::Greater => y -= 1,
        //         std::cmp::Ordering::Less => x += 1,
        //         std::cmp::Ordering::Equal => {
        //             answer.push(vec![x, y]);
        //             y -= 1;
        //             x += 1;
        //         }
        //     }
        // }
        // answer

        vec![] // 仅仅是为了让编译通过，因为题目没有给出CustomFunction的实现
    }
}