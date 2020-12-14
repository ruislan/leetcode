use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // 方法1
        // 直接hashmap存储
        // key为每个排序后的字符，value为字符本身
        // 然后把map转换成二维数组即可
        // 由于有排序，时间是O(n^2logn)，空间是O(n)
        // Passed 12ms 4.6mb
        // let mut map = std::collections::HashMap::new();
        // for s in strs {
        //     let mut chars: Vec<char> = s.chars().collect();
        //     chars.sort();
        //     let key: String = chars.into_iter().collect();
        //     map.entry(key).or_insert(Vec::new()).push(s);
        // }
        // map.into_iter().map(|(k, v)| v).collect()

        // 方法2
        // 由于Rust对于hash的处理会稍微重一点
        // 再加上只有小写字母，那么直接用桶排序吧，
        // 这样时间就是O(n^2)，空间会多一些空桶的位置
        // Passed 8ms 5.4mb
        let mut map = std::collections::HashMap::new();
        for s in strs {
            let mut chars = vec![0; 26];
            s.bytes().for_each(|x| chars[(x - 97) as usize] += 1);
            map.entry(chars).or_insert(Vec::new()).push(s);
        }
        map.into_iter().map(|(_, v)| v).collect()
    }
}