use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        // 方法1
        // let mut groups = vec![0;10000];
        // for i in 0..deck.len() {
        //     groups[deck[i] as usize] += 1;
        // }
        //
        // groups.retain(|&x| x > 0 );
        // let mut max = groups.iter().max().unwrap();
        //
        // for k in 2..=*max {
        //     let mut count = 0;
        //     for num in &groups {
        //         if num % &k == 0 { count += 1; }
        //     }
        //     if count == groups.len() { return true; }
        // }
        // false


        // 方法2
        fn gcd(a: i32, b: i32) -> i32 {
            if a == 0 { b } else { gcd(b % a, a) }
        }

        let mut groups = vec![0; 10000];
        for i in 0..deck.len() {
            groups[deck[i] as usize] += 1;
        }

        let mut g = -1;
        for num in groups {
            if num > 0 {
                if g == -1 { g = num; } else {
                    g = gcd(g, num);
                }
            }
        }
        g >= 2
    }
}