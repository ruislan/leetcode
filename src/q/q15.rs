use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // 方法1
        // 核心思想就是首先取一个记为one，剩下两个和就是0 - one
        // 然后在剩下的数组中取一个记为two，剩下那个就是0 - two，记为three
        // 这样我们会形成O(n^3)必然超时
        // 所以我们需要首先将nums排序，然后利用有序的特点，用二分搜索来处理three
        // 这样我们会优化到O(nlogn + n^2logn)
        // AC 432ms 3.5mb
        // let mut nums = nums;
        // nums.sort_unstable();
        // let n = nums.len();
        // let mut answer = std::collections::HashSet::new();
        // for i in 0..n {
        //     if nums[i] > 0 { break; }
        //     let two = 0 - nums[i];
        //     for j in i + 1..n {
        //         let three = two - nums[j];
        //         match nums[j + 1..].binary_search(&three) {
        //             Ok(_) => answer.insert(vec![nums[i], nums[j], three]),
        //             Err(_) => false,
        //         };
        //     }
        // }
        // answer.into_iter().collect()

        // 方法2
        // 利用hashmap来存储数字three和它的位置，
        // 如果位置在两个位置后面，那么就说明这个数字可用
        // AC 476ms 3.8mb
        // let mut nums = nums;
        // nums.sort_unstable();
        // let n = nums.len();
        // let mut map = std::collections::HashMap::new();
        // for i in 0..n {
        //     map.entry(nums[i]).or_insert(Vec::new()).push(i);
        // }
        //
        // let mut answer = std::collections::HashSet::new();
        // for i in 0..n {
        //     if nums[i] > 0 { break; }
        //     let two = 0 - nums[i];
        //     for j in i + 1..n {
        //         let three = two - nums[j];
        //         if let Some(places) = map.get(&three) {
        //             if let Some(&last) = places.last() {
        //                 if last > j {
        //                     answer.insert(vec![nums[i], nums[j], three]);
        //                 }
        //             }
        //         }
        //     }
        // }
        // answer.into_iter().collect()

        // 方法3
        // 前两个方法我们做了大量的hash运算，在rust中是很昂贵的
        // 这次我们换成双指针来处理两数和的问题
        // 核心思想还是没变，我们先排序，这样可以避免一些重复
        // 然后我们找到第一个数字，one,如果这个数字，我们和我们之前处理的相同，那么我们可以不用处理了
        // 然后我们要找两数字的和设置为target，第二层循环，我们使用双指针
        // 如果左右指针的和大于target，那么降低右指针，
        // 如果左右指针的和小于target，那么增加左指针，
        // 如果左右指针的和等于target，那么进入结果集，那么这两个数字已经被使用了
        //    我们增加左指针，降低右指针，同时，我们要避免左右是重复的数字，所以
        //    我们左指针要一直增加到是新的数字，右指针相似的操作
        // AC 24ms 3.4mb
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len();

        let mut answer = Vec::new();
        for i in 0..n {
            if i > 0 && nums[i] == nums[i - 1] { continue; }
            let target = 0 - nums[i];
            let mut hi = n - 1;
            let mut lo = i + 1;
            while lo < hi {
                let sum = nums[lo] + nums[hi];
                if sum > target {
                    hi -= 1;
                } else if sum < target {
                    lo += 1;
                } else {
                    answer.push(vec![nums[i], nums[lo], nums[hi]]);
                    hi -= 1;
                    lo += 1;
                    while lo < hi && nums[lo] == nums[lo - 1] {
                        lo += 1;
                    }
                    while lo < hi && nums[hi] == nums[hi + 1] {
                        hi -= 1;
                    }
                }
            }
        }
        answer
    }
}