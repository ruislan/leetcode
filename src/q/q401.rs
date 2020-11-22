use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn read_binary_watch(num: i32) -> Vec<String> {
        let mut res = Vec::new();
        let num = num as u32;
        for hour in 0..12u32 {
            for minute in 0..60u32 {
                if hour.count_ones() + minute.count_ones() == num {
                    let mut time = hour.to_string() + ":";
                    if minute < 10 {
                        time += "0"
                    }
                    time += &minute.to_string();
                    res.push(time);
                }
            }
        }
        res
    }
}