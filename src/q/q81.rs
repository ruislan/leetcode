use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        // 方法1
        // 旋转了数组相当于把一个升序数组分成了两个升序数组
        // 而nums[0]就正好是分界点，把大于等于nums[0]的叫big数组
        // 有序数组查找二分法是必须的
        // 首先我们要知道target是否在big数组中，可以通过target >= nums[0]
        // 然后我们要知道我们的在搜索的范围是否在big数组中，可以通过nums[mid] >= nums[0]来判断
        // 如果target和search都在同一个区域，那么就可以采用正常的二分去处理
        // 如果target和search不在同一个区域，那么就看search在哪个区域，
        // 如果search在big区域，我们就应该向右搜索也就是lo = mid + 1
        // 如果search不在big区域，我们就应该向左搜索也就是hi = mid - 1
        // （动手画一画上述过程）
        // 上面的过程适合没有重复的情况，
        // 那么现在来解决重复的问题
        // 重复的问题，我们可以通过去掉重复来解决
        // 判断当前位置，也就是nums[mid]与当前区域的左边界lo是否相等，如果相等则lo + 1也就是向右缩小范围
        // 如果nums[mid]与当前区域的右边界hi是否相等，如果相等则hi - 1也就是向左缩小范围
        // 这样就能形成一个没有重复的情况
        // AC 0ms 2mb
        let n = nums.len();
        let mut lo = 0;
        let mut hi = nums.len() - 1;
        while lo <= hi && hi < n {
            let mid = lo + (hi - lo) / 2;
            let value = nums[mid];
            if value == target { return true; }

            if value == nums[lo] { lo += 1; continue; }
            if value == nums[hi] { hi -= 1; continue; }

            let search_big = value >= nums[0];
            let target_big = target >= nums[0];
            if (search_big == target_big) {
                if value >= target {
                    hi = mid - 1;
                } else {
                    lo = mid + 1;
                }
            } else {
                if search_big {
                    lo = mid + 1;
                } else {
                    hi = mid - 1;
                }
            }
        }
        false
    }
}
