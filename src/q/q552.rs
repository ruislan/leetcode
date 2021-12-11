use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn check_record_2(n: i32) -> i32 {
        // 方法1
        // 回溯 + 记忆化
        // AC 340ms 29.3mb 用hashmap会超时：(
        fn dfs(i: i32, n: i32, s: &mut Vec<char>, cnt_a: i32, sec_l: i32, mem: &mut Vec<Vec<Vec<i32>>>) -> i32 {
            if s.len() as i32 == n { return 1; }
            if mem[i as usize][cnt_a as usize][sec_l as usize] > -1 { return mem[i as usize][cnt_a as usize][sec_l as usize]; }

            let _mod = 1000000007;
            let mut sum = 0;

            // p
            s.push('P');
            sum += dfs(i + 1, n, s, cnt_a, 0, mem);
            sum = sum % _mod;
            s.pop();

            // a
            if cnt_a < 1 {
                s.push('A');
                sum += dfs(i + 1, n, s, cnt_a + 1, 0, mem);
                sum = sum % _mod;
                s.pop();
            }

            // l
            if sec_l < 2 {
                s.push('L');
                sum += dfs(i + 1, n, s, cnt_a, sec_l + 1, mem);
                sum = sum % _mod;
                s.pop();
            }
            mem[i as usize][cnt_a as usize][sec_l as usize] = sum;
            sum
        }
        dfs(0, n, &mut Vec::new(), 0, 0, &mut vec![vec![vec![-1; 3]; 2]; n as usize])
    }
}