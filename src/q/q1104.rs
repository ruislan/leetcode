use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
        // 方法1
        // 由于是无限的完全二叉树
        // 所以这里我们只需要找到label所在的数组中的下标位置
        // 如果下标是偶数就可以通过2x+2=idx求得，它的父节点的下标位置
        // 如果下标是奇数就可以通过2x+1=idx求得
        // 直到下标位置为0为止
        // AC 8ms 7.4mb
        // PS. 还可以优化，后面有时间来吧
        let mut tree = Vec::new();
        let mut answer = Vec::new();
        let label = label as u32;
        let mut level = 1;
        let mut idx = 0;
        loop {
            let start = 2_u32.pow(level - 1);
            let limit = 2_u32.pow(level);
            if level & 1 == 1 {
                for i in start..limit {
                    tree.push(i);
                    idx += 1;
                    if label == i { break; }
                }
            } else {
                for i in (start..limit).rev() {
                    tree.push(i);
                    idx += 1;
                    if label == i { break; }
                }
            }
            if limit > label { break; }
            level += 1;
        }
        idx -= 1;
        while idx > 0 {
            answer.push(tree[idx] as i32);
            if idx & 1 == 0 {
                idx = (idx - 2) >> 1;
            } else {
                idx = (idx - 1) >> 1;
            }
        }
        answer.push(1);
        answer.reverse();
        answer
    }
}

