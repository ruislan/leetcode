use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        for i in 0..nums.len() {
            let mut e = map.entry(nums[i]).or_insert((0, i, 0));
            e.0 += 1;
            e.2 = i;
        }

        let mut max = 0;
        for (_, v) in &map {
            if v.0 > max { max = v.0; }
        }

        let mut min_len = 0;
        for (_, v) in &map {
            if v.0 == max {
                let cur_len = v.2 - v.1 + 1;
                if min_len == 0 || cur_len < min_len { min_len = cur_len; }
            }
        }
        min_len as i32
    }
}