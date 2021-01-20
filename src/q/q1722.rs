use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn minimum_hamming_distance(source: Vec<i32>, target: Vec<i32>, allowed_swaps: Vec<Vec<i32>>) -> i32 {
        // 方法1
        // 将所有可以交换的看成一个联通的网络
        // 在这个网络中所有的元素都是可以自由放在任意位置的
        // 然后网络外的元素考虑相同位置是否不同
        // 网络中的元素不考虑位置只考虑不同的
        // 最后这两个不同就是结果
        fn find(mut x: usize, parents: &mut Vec<usize>) -> usize {
            if x != parents[x] {
                parents[x] = find(parents[x], parents);
            }
            parents[x]
        }

        fn union(mut x: usize, mut y: usize, parents: &mut Vec<usize>) -> bool {
            let mut root_x = find(x, parents);
            let mut root_y = find(y, parents);
            if root_x == root_y { return false; }
            parents[root_y] = root_x;
            true
        }

        let n = source.len();
        let mut parents = vec![0; n as usize];
        for i in 0..n as usize { parents[i] = i; }

        let mut visited = std::collections::HashSet::new();
        for allow in allowed_swaps {
            if union(allow[0] as usize, allow[1] as usize, &mut parents) {
                visited.insert(allow[0] as usize);
                visited.insert(allow[1] as usize);
            }
        }

        let mut answer = 0;
        let mut common = std::collections::HashMap::new();
        for i in 0..n {
            if visited.contains(&i) {
                *common.entry(source[i]).or_insert(0_i32) += 1;
                *common.entry(target[i]).or_insert(0_i32) -= 1;
            } else if source[i] != target[i] {
                answer += 1;
            }
        }
        let mut diff = 0;
        for (_, v) in common {
            if v != 0 { diff += v.abs(); }
        }
        answer + diff / 2
    }
}