use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // 方法1
        // 我们先按照h进行排序，也就是确定一个未知因素
        // 那么就剩下根据k来调整了
        // 创建结果数组answer
        // 迭代people：
        //    当people[i]的k是0的话：
        //        那么只需要从answer中找到第一个空位即可放入
        //    当people[i]的k是1的话：
        //        那么需要从answer中略过1个空或者1个比它大或等的值，之后的空位放入
        //    依次类推
        // 最后返回answer
        // Passed 12ms 2.1mb time:O(n^2)
        let mut people = people;
        let mut n = people.len();
        people.sort_unstable();
        let mut answer = vec![Vec::new(); n];
        for p in people {
            let mut count = 0;
            for i in 0..n {
                if answer[i].is_empty() || p[0] <= answer[i][0] { count += 1; }
                if answer[i].is_empty() && count > p[1] {
                    answer[i] = p;
                    break;
                }
            }
        }
        answer

        // 方法2
        // 我们按照最高的人在最前面的方式排序，这样排序后的结果就是h高到低，k低到高
        // 然后我们依次插入到结果数组中
        // Passed 4ms 2mb O(nlogn)
        // let mut people = people;
        // people.sort_unstable_by(|a, b| {
        //     let ord = b[0].cmp(&a[0]);
        //     if ord == std::cmp::Ordering::Equal { a[1].cmp(&b[1]) } else { ord }
        // });
        // let mut answer = Vec::new();
        // for p in people {
        //     answer.insert(p[1] as usize, p);
        // }
        // answer
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::reconstruct_queue(vec![vec![7, 0], vec![4, 4], vec![7, 1], vec![5, 0], vec![6, 1], vec![5, 2]]),
        vec![vec![5, 0], vec![7, 0], vec![5, 2], vec![6, 1], vec![4, 4], vec![7, 1]]
    )
}