use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn interval_intersection(
        first_list: Vec<Vec<i32>>,
        second_list: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        // 方法1
        // 我们分解一下问题，可以得到两个问题
        // 第一，我们如何判断相交
        // 第二，我们如何迭代两个数组
        // 第一个问题，我们如何判断相交
        // 那么我们可以得到四个点分别为first的起止点,s1,e1和second的起止点s2,e2
        // 如果其中一个区间的终点比另一个起点还小，也即是e2 < s1, e1 < s2，那么这两个不相交
        // 剩下的都有相交的地方，那么相交的区间，我们只需要取max(s1, s2)和min(e1, e2)就是相交的区间
        // 那么第二个问题，该如何迭代呢？
        // 这里需要从第一个问题来看，第一个问题产生了两个结果，就是相交和不相交
        // 不相交的区间，那么要迭代first还是second，我们就看谁的end比较小，也就是把落后的那个区间向前迭代一个
        // （这里注意的是区间**已经排序**，这个很重要，如果没有排序，那就做一个排序。）
        // 而相交的区间，同样的，我们也迭代end比较小的那个
        // AC 8ms 2.2mb
        let n1 = first_list.len();
        let n2 = second_list.len();
        let mut answer = Vec::new();
        let mut p1 = 0;
        let mut p2 = 0;
        while p1 < n1 && p2 < n2 {
            let v1 = &first_list[p1];
            let v2 = &second_list[p2];
            let (s1, e1) = (v1[0], v1[1]);
            let (s2, e2) = (v2[0], v2[1]);
            if e1 < s2 {
                p1 += 1;
                continue;
            }
            if e2 < s1 { 
                p2 += 1;
                continue;
            }
            answer.push(vec![s1.max(s2), e1.min(e2)]);
            if e1 < e2 { 
                p1 += 1;
            } else {
                p2 += 1;
            }
        }
        answer
    }
}
