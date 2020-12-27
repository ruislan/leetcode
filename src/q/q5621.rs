use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        // 方法1
        // 统计students的喜好数量，ones和zeros
        // 迭代sandwiches，出现的口味对应students的喜好数量减少1
        // 直到喜好一种口味的剩余学生为0，但是还有这个口味的sandwich时，
        // 返回剩余学生数，即是ones + zeros
        let mut ones = students.iter().filter(|&&x| x == 1).count() as i32;
        let mut zeros = students.iter().filter(|&&x| x == 0).count() as i32;
        for x in sandwiches {
            if x == 1 {
                if ones == 0 { break; }
                ones -= 1;
            } else {
                if zeros == 0 { break; }
                zeros -= 1;
            }
        }
        ones + zeros
    }
}