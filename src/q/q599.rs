use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        // 方法1
        // 暴力法
        // 迭代list1，然后迭代list2
        // 当list1[i] == list2[j]，取得索引和
        // 将索引和与最小索引和min比较，
        //     如果相等： 添加进answer
        //     如果小：   清空answer，将这个结果放入answer
        // 迭代完成，返回answer
        // Passed 24ms 2.5mb
        // let mut min = list1.len() + list2.len();
        // let mut answer = Vec::new();
        // for (i, s1) in list1.iter().enumerate() {
        //     for (j, s2) in list2.iter().enumerate() {
        //         if s1 == s2 {
        //             let sum = i + j;
        //             if sum <= min {
        //                 if sum < min { answer.clear(); }
        //                 answer.push(s1.clone());
        //                 min = sum;
        //             }
        //         }
        //     }
        // }
        // answer

        // 方法2
        // hashmap，将list1的索引和对应的值放入hashmap
        // 迭代list2，如果能找到hashmap中，取得索引和
        // 将索引和与最小索引和min比较，
        //     如果相等： 添加进answer
        //     如果小：   清空answer，将这个结果放入answer
        // 迭代完成，返回answer
        // Passed 12ms 2.3mb
        let mut min = list1.len() + list2.len();
        let mut answer = Vec::new();
        let mut map1 = std::collections::HashMap::new();
        for (i, x) in list1.into_iter().enumerate() {
            map1.entry(x).or_insert(i);
        }
        for (j, y) in list2.into_iter().enumerate() {
            if let Some(i) = map1.get(&y) {
                let sum = *i + j;
                if sum <= min {
                    if sum < min { answer.clear(); }
                    answer.push(y);
                    min = sum;
                }
            }
        }
        answer
    }
}

#[test]
fn test() {
    let v: Vec<String> = vec![];
    assert_eq!(Solution::find_restaurant(vec![], vec![]), v);

    assert_eq!(Solution::find_restaurant(
        vec!["Shogun", "Tapioca Express", "Burger King", "KFC"].into_iter().map(|x| x.to_string()).collect::<Vec<String>>(),
        vec!["Piatti", "The Grill at Torrey Pines", "Hungry Hunter Steakhouse", "Shogun"].into_iter().map(|x| x.to_string()).collect::<Vec<String>>()),
               vec!["Shogun"].into_iter().map(|x| x.to_string()).collect::<Vec<String>>()
    );

    assert_eq!(Solution::find_restaurant(
        vec!["Shogun", "Tapioca Express", "Burger King", "KFC"].into_iter().map(|x| x.to_string()).collect::<Vec<String>>(),
        vec!["KFC", "Shogun", "Burger King"].into_iter().map(|x| x.to_string()).collect::<Vec<String>>()),
               vec!["Shogun"].into_iter().map(|x| x.to_string()).collect::<Vec<String>>()
    );

    let answer = Solution::find_restaurant(
        vec!["Shogun", "KFC", "Burger King", "Tapioca Express"].into_iter().map(|x| x.to_string()).collect::<Vec<String>>(),
        vec!["KFC", "Shogun", "Burger King"].into_iter().map(|x| x.to_string()).collect::<Vec<String>>());
    assert_eq!(answer == vec!["Shogun", "KFC"].into_iter().map(|x| x.to_string()).collect::<Vec<String>>() ||
                   answer == vec!["KFC", "Shogun"].into_iter().map(|x| x.to_string()).collect::<Vec<String>>(), true);
}