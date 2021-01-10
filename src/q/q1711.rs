use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn count_pairs(deliciousness: Vec<i32>) -> i32 {
        // 方法1
        // 和q1的思想类似，无需赘述了
        // 需要注意2点，
        // 1：就是不需要取到2^31，因为限制是在2^20,所以只需要到2^20 + 2^20 = 2^21即可，到31还因为i32是负数而出错
        // 2：就是不要去找比deliciousness[i]小的pows，因为deliciousness取值是>=0的，不会有负数存在
        let m = 10_i32.pow(9) + 7;
        let mut hashmap = std::collections::HashMap::new();
        let pows: Vec<i32> = (0..22).map(|i| 2_i32.pow(i)).collect();
        let mut answer = 0;
        for x in deliciousness {
            for i in 0..22 {
                if pows[i] >= x {
                    let y = pows[i] - x;
                    if let Some(&n) = hashmap.get(&y) {
                        answer += n;
                        answer = answer % m;
                    }
                }
            }
            *hashmap.entry(x).or_insert(0) += 1;
        }
        answer
    }
}