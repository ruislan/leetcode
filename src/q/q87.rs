use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        // 方法1
        // 这个算是分治了吧
        // 遇到困难题，可以有这么样一个思路，那就是先简化和分拆问题
        // 例如这道题描述中有交换，a+b = b+a，那么我们可以先忽略掉这个交换
        // 这样一来，这个递归就比较好写了，我们持续从索引1开始分割字符串s1为a1,b1，s2为a2,b2
        // 如果在i位置分割出来的a1[..i] == a2[..i] && b1[i..] == b2[i..]，那么就意味着这个分割是OK的
        // 现在我们来处理交换的问题，看起来也不复杂了，在i位置切割的话，s2就在n - i位置切割
        // 这样一来就检查a1[..i] == a2[n-i..] && b1[i..] == b2[..n-i] 
        // 例如： great rgeat，i=2的时候
        // great => gr|eat; rgeat => rg|eat; 和 great => gr|eat; rgeat => rge|at;
        // 到这里，这个问题至少我们解决了一大部分
        // 但是，如果提交的话，会超时，主要超时在哪里呢？
        // 在我们会做大量的重复计算，例如：great和rgeta
        // gr|eat:rg|eta的时候我们会对比gr和rg，而gre|at:rge|ta的时候，我们还会对比gr和rg
        // 而这样的对比有很多，所以我们要剪枝，利用记忆化来记住已经比对过的结果
        // 这样就不会超时了
        // AC 4ms 2.1mb
        use std::collections::HashMap;
        fn daq(s1: String, s2: String, mem: &mut HashMap<(String, String), bool>) -> bool {
            if let Some(&eq) = mem.get(&(s1.clone(), s2.clone())) {
                return eq;
            }

            if s1 == s2 {
                mem.insert((s1.clone(), s2.clone()), true);
                return true;
            }

            let mut f = vec![0; 26];
            for ch in s1.bytes() {
                f[ch as usize - 97] += 1;
            }
            for ch in s2.bytes() {
                let i = ch as usize - 97;
                f[i] -= 1;
                if f[i] < 0 {
                    mem.insert((s1.clone(), s2.clone()), false);
                    return false;
                }
            }

            let n = s1.len();
            for i in 1..n {
                if daq(s1[0..i].to_string(), s2[0..i].to_string(), mem)
                    && daq(s1[i..].to_string(), s2[i..].to_string(), mem)
                {
                    mem.insert((s1.clone(), s2.clone()), true);
                    return true;
                }
                if daq(s1[0..i].to_string(), s2[n - i..].to_string(), mem)
                    && daq(s1[i..].to_string(), s2[0..n - i].to_string(), mem)
                {
                    mem.insert((s1.clone(), s2.clone()), true);
                    return true;
                }
            }
            mem.insert((s1.clone(), s2.clone()), false);
            false
        }

        daq(s1, s2, &mut HashMap::new())
    }
}
