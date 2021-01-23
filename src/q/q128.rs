use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        // 方法1
        // 先去重再排序
        // 然后计算连续的即可
        // 但是时间是O(nlogn)
        // let mut sets = std::collections::HashSet::new();
        // for n in nums { sets.insert(n); }
        // let mut nums: Vec<i32> = sets.into_iter().collect();
        // nums.sort();
        // let mut answer = 0;
        // let mut count = 1;
        // for i in 1..nums.len() {
        //     if nums[i - 1] + 1 == nums[i] {
        //         count += 1;
        //     } else {
        //         count = 1;
        //     }
        //     answer = answer.max(count);
        // }
        // answer

        // 方法2
        // 并查集
        // 先对数组去重
        // 连续的就是一个集合，我们只需要找出nums[i] + 1与nums[i]相连即可
        // 当然nums[i] + 1要在数组中
        fn find(x: usize, parent: &mut Vec<usize>) -> usize {
            if x != parent[x] { parent[x] = find(parent[x], parent); }
            parent[x]
        }

        fn unite(x: usize, y: usize, parent: &mut Vec<usize>, sz: &mut Vec<i32>) {
            let root_x = find(x, parent);
            let root_y = find(y, parent);
            if root_x == root_y { return; }
            parent[root_y] = root_x;
            sz[root_x] += sz[root_y];
        }

        let mut sets = std::collections::HashSet::new();
        for n in nums { sets.insert(n); }
        let mut nums: Vec<i32> = sets.into_iter().collect();
        let n = nums.len();
        let mut parent = vec![0; n];
        for i in 0..n { parent[i] = i; }
        let mut sz = vec![1; n];
        let mut map = std::collections::HashMap::new();
        for i in 0..n { map.insert(nums[i], i); }

        for i in 0..n {
            if let Some(&j) = map.get(&(nums[i] + 1)) {
                unite(i, j, &mut parent, &mut sz);
            }
        }

        sz.into_iter().max().unwrap()
    }
}