use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        // 方法1
        // 迭代数组arr1，迭代数组arr2，如果不存在|arr1[i] - arr2[j]| <= d，计数加1
        // 返回计数值
        // O(n^2)
        // Passed 0ms 2.1mb
        // arr1.iter().filter(|&&n1| arr2.iter().all(|&n2| (n1 - n2).abs() > d)).count() as i32

        // 方法2
        // 排序arr2，然后二分查找最接近或者相等于arr1[i]的值，如果这个值距离大于d，则计数加1
        // 返回计数值
        // O(nlogn)
        // Passed 0ms 2.1mb
        let mut arr2 = arr2;
        arr2.sort_unstable();
        println!("{:?}", arr2);
        arr1.iter().filter(|&n1| {
            let j = arr2.binary_search(n1).unwrap_or_else(|x| x);
            let j = std::cmp::min(j, arr2.len() - 1);
            std::cmp::min((*n1 - arr2[j.saturating_sub(1)]).abs(), (*n1 - arr2[j]).abs()) > d
        }).count() as i32
    }
}

#[test]
fn test_q1385() {
    // assert_eq!(Solution::find_the_distance_value(vec![], vec![], 0), 0);
    // assert_eq!(Solution::find_the_distance_value(vec![4, 5, 8], vec![10, 9, 1, 8], 2), 2);
    // assert_eq!(Solution::find_the_distance_value(vec![1, 4, 2, 3], vec![-4, -3, 6, 10, 20, 30], 3), 2);
    // assert_eq!(Solution::find_the_distance_value(vec![2, 1, 100, 3], vec![-5, -2, 10, -3, 7], 6), 1);
    assert_eq!(Solution::find_the_distance_value(
        vec![-803, 715, -224, 909, 121, -296, 872, 807, 715, 407, 94, -8, 572, 90, -520, -867, 485, -918, -827, -728, -653, -659, 865, 102, -564, -452, 554, -320, 229, 36, 722, -478, -247, -307, -304, -767, -404, -519, 776, 933, 236, 596, 954, 464],
        vec![817, 1, -723, 187, 128, 577, -787, -344, -920, -168, -851, -222, 773, 614, -699, 696, -744, -302, -766, 259, 203, 601, 896, -226, -844, 168, 126, -542, 159, -833, 950, -454, -253, 824, -395, 155, 94, 894, -766, -63, 836, -433, -780, 611, -907, 695, -395, -975, 256, 373, -971, -813, -154, -765, 691, 812, 617, -919, -616, -510, 608, 201, -138, -669, -764, -77, -658, 394, -506, -675, 523, 730, -790, -109, 865, 975, -226, 651, 987, 111, 862, 675, -398, 126, -482, 457, -24, -356, -795, -575, 335, -350, -919, -945, -979, 611],
        37), 0);
}