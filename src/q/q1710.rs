use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        // 方法1
        // 贪婪算法
        // 将箱子通过单元数从大到小的顺序排序
        // 然后一直拿单元数最多的那个箱子，直到到达truck_size
        // 返回结果
        let mut box_types = box_types;
        box_types.sort_unstable_by(|a, b| b[1].cmp(&a[1]));
        let mut answer = 0;
        let mut count = 0;
        for boxes in box_types {
            for _ in 0..boxes[0] {
                count += 1;
                if count <= truck_size {
                    answer += boxes[1];
                }
            }
        }
        answer
    }
}