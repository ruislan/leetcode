use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        // 方法1
        // 排序arr为o_arr，删除重复的值，然后迭代o_arr将index和数字放入hashmap中
        // 迭代arr，查询每个值的位置，放入vec中返回
        // Passed 28-32ms 5.1mb
        // let mut o_arr = arr.clone();
        // o_arr.sort_unstable();
        // o_arr.dedup();
        // let mut map = std::collections::HashMap::new();
        // (0..o_arr.len()).for_each(|i| { map.insert(o_arr[i], i as i32 + 1); });
        // arr.iter().map(|x| *map.get(x).unwrap_or(&0)).collect();

        // 方法2
        // 建立一个桶，找到这个桶的长度（arr[i]的max - min + 1），
        // 然后将arr[i]作为下标存储到桶的位置bag[arr[i] - min]中
        // 然后计算前序和，作为桶的该元素的实际排序位置
        // 然后根据arr[i] - min作为bag的下标查询实际的排序位置作为返回
        // Passed 16-24ms 6.6mb
        let (mut max, mut min) = (i32::min_value(), i32::max_value());
        for i in 0..arr.len() {
            if arr[i] > max { max = arr[i]; }
            if arr[i] < min { min = arr[i]; }
        }
        let mut bag = vec![0; (max - min) as usize + 1];
        (0..arr.len()).for_each(|i| { bag[(arr[i] - min) as usize] = 1; });
        (1..bag.len()).for_each(|i| { bag[i] += bag[i - 1]; });
        arr.iter().map(|&x| bag[(x - min) as usize]).collect()
    }
}

#[test]
fn test_q1331() {
    // assert_eq!(Solution::array_rank_transform(vec![]), vec![]);
    assert_eq!(Solution::array_rank_transform(vec![40, 10, 20, 30]), vec![4, 1, 2, 3]);
    assert_eq!(Solution::array_rank_transform(vec![100, 100, 100]), vec![1, 1, 1]);
    assert_eq!(Solution::array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12]), vec![5, 3, 4, 2, 8, 6, 7, 1, 3]);
}