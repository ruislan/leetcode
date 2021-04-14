use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        // 方法1
        // HashSet方法，这个很实在，但是就是会有额外的空间
        // 这个就不赘述了

        // 方法2
        // floyd成环检测，也叫龟兔赛跑算法，
        // 也就是快慢指针，慢指针走1步，快指针走2步，
        // 两个指针一定会在环内相遇
        // 然后将慢指针指向出口，快慢指针再以相同速度走
        // 相遇的地点就是环的入口
        // 例如:
        // index: 0 1 2 3 4 
        // value: 1 3 4 2 2
        // slow: (0)=1,     (3)=4,   (4)=2,    
        // fast: ((0)=1)=3,((3)=4)=2,((2)=4)=2
        // slow和fast在值=2的地方相遇
        // slow: (0) = 1, (1)=3, (3)=2
        // fast: 2      , (2)=4, (4)=2
        // 快慢指针在值等于2的地方相遇，所以2就是重复的那个数字
        // AC 0ms 2.2mb
        let mut p0 = nums[0];
        let mut p1 = nums[0];
        loop {
            p0 = nums[p0 as usize];
            p1 = nums[nums[p1 as usize] as usize];
            if p0 == p1 { break; }
        }
        p0 = nums[0];
        while p0 != p1 {
            p0 = nums[p0 as usize];
            p1 = nums[p1 as usize];
        }
        p0
    }
}