use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        // 方法1
        // 首先迭代edges，将关系建立起来，形成类似(0:(1,2)),(1:0),(2:(0,3,4,5)),(3:2),(4:2),(5:2)的结构
        // 然后从0到n开始迭代，每次使用广度优先的方式查找所有的节点，然后统计边的距离
        // 例如，
        // 0开始，0的后续节点是1，2，这是一度邻居，所以总和是1+1，将0放入已经处理的节点中，然后把1，2放入队列，且度数变成2度，
        // 继续循环
        // 然后取出1，1的后续节点是0，而0已经处理过了（用hashset来存储已经处理过的节点），那么这个点不再处理，将1放入已经处理的节点中
        // 然后取出2，2的后续节点是0，3，4，5，而0已经处理过了，然后是3，4，5，度数是2，那么和是2+2+2，然后将2放入已经处理的节点中，3，4，5放入队列，且度数变为3，
        // 继续循环
        // 然后取出3，3的后续节点是2，2已经处理过了，不处理，
        // ……
        // 然后取出5，5的后续节点是2，2已经处理过了，不处理，
        // 所有节点处理完了，（0..n）的迭代进入 1，然后使用广度优先处理1的节点，
        // 依次类推，直到所有节点都处理完
        // O(n^2)
        // Not Passed 64/69 数据在1万个时会超时
        // TODO 先这样，后面优化后再重新写答案
        use std::collections::*;
        let mut graph = HashMap::new();
        edges.into_iter().for_each(|v| {
            let neighbours = graph.entry(v[0]).or_insert(Vec::new());
            if v.len() > 1 {
                neighbours.push(v[1]);
                let neighbours = graph.entry(v[1]).or_insert(Vec::new());
                neighbours.push(v[0]);
            }
        });

        let mut res = Vec::new();
        for i in 0..n {
            let mut skips = HashSet::new();
            let mut queue = VecDeque::new();
            let mut levels = 1;
            let mut sum = 0;
            queue.push_back(i);
            while !queue.is_empty() {
                let mut sub_sum = 0;
                for _ in 0..queue.len() {
                    let node = queue.pop_front().unwrap();
                    for neighbours in graph.get(&node) {
                        for neighbour in neighbours {
                            if !skips.contains(neighbour) {
                                sub_sum += levels;
                                queue.push_back(*neighbour);
                            }
                        }
                    }
                    skips.insert(node);
                }
                levels += 1;
                sum += sub_sum;
            }
            res.push(sum);
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::sum_of_distances_in_tree(2, vec![vec![0, 1]]),
        vec![1, 1]
    );
    assert_eq!(
        Solution::sum_of_distances_in_tree(6, vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4], vec![2, 5]]),
        vec![8, 12, 6, 10, 10, 10]
    );
}