struct Solution;

impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        // 方法1
        // 暴力解决，迭代0..nums.len()，然后按照index[i]的位置插入数组，值为nums[i]
        // 这里涉及到大量的数组移位操作，所以是暴力法（输入数据量太小，暴力也适用了）
        // Passed 0ms 2mb
        let mut arr = Vec::with_capacity(nums.len());
        (0..nums.len()).for_each(|i| {
            arr.insert(index[i] as usize, nums[i]);
        });
        arr

        // 方法2
        // 构建一个数组arr，值为stack，所有相同的位置的都存入同一个stack中
        // 迭代数组arr，然后每个stack按照后进先出的规则放到新的数组res中，返回res
        // 这里都是push，没有了数组移位操作
        // 这个思路错误！！！！！Not Passed
        // 因为插入后的数字改变了数组中后面数字的位置，所以这个思路在某些情况下会失效，例如0,0,1,3,1
        // 看了一下，官方也没有更好的方法，也只有移位操作
        // let mut arr = vec![vec![]; nums.len()];
        // (0..nums.len()).for_each(|i| {
        //     arr[index[i] as usize].push(nums[i]);
        // });
        // arr.into_iter().filter(|v| !v.is_empty()).map(|mut v| {
        //     v.reverse();
        //     v
        // }).flatten().collect()
    }
}

#[test]
fn test() {
    // assert_eq!(Solution::create_target_array(vec![], vec![]), vec![]);
    // assert_eq!(Solution::create_target_array(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1]), vec![0, 4, 1, 3, 2]);
    // assert_eq!(Solution::create_target_array(vec![1, 2, 3, 4, 0], vec![0, 1, 2, 3, 0]), vec![0, 1, 2, 3, 4]);
    // assert_eq!(Solution::create_target_array(vec![1], vec![0]), vec![1]);
    assert_eq!(Solution::create_target_array(vec![4, 2, 4, 3, 2], vec![0, 0, 1, 3, 1]), vec![2, 2, 4, 4, 3]);
}