use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        // 方法1
        // 贪心算法+hashmap
        // 这里首先我们利用hashmap统计说了其他兔子🐰数量相同的为1组
        // 然后我们迭代这个分组，
        // 首先分组为0的这个必然每个都是不同的颜色，这个应该很好理解
        // 然后我们思考可以发现，如果数组中包含了全部的颜色的兔子，那么分组必然是分组号+1的倍数
        // 例如，1号分组，那么必然是2只，4只，6只这样的倍数正好，如果余数是1只，那么说明至少有1只不在answers中
        // 再如，2号分组，那么必然是3只，6只，9只这样的倍数正好，如果余数是1只，那么说明至少有2只不在answers中
        // 所以我们求得倍数和余数，如果余数为0，那么表示所有的兔子都在里面，那么(编号+1) * 倍数正好
        // 如果余数不为0，那么表示除了倍数的兔子都在answers里面以外，还有（编号+1）只兔子中的部分兔子不在answers
        // 所以我们再加上这个（编号+1）只兔子即可。
        // AC 0ms 1.9mb
        let mut freq = vec![0; 1000];

        for i in 0..answers.len() {
            freq[answers[i] as usize] += 1;
        }

        let mut answer = freq[0];
        for i in 1..freq.len() {
            let multiple = freq[i] / (i + 1);
            answer += (i + 1)
                * if freq[i] % (i + 1) == 0 {
                    multiple
                } else {
                    multiple + 1
                };
        }
        answer as i32
    }
}
