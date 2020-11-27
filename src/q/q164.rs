use crate::q::Solution;

impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        // 方法1
        // 库函数排序 + 逐个检查即可
        // Passed 0ms 2.1mb 时间O(nlogn)，空间O(1)
        // 这样做其实不符合题目的要求，题目要求时间O(n)，空间O(n)
        // if nums.len() < 2 { return 0; }
        // let mut nums = nums;
        // nums.sort_unstable();
        // nums.windows(2).map(|x| x[1] - x[0]).max().unwrap()

        // O(n)的排序能想到的只有一种，那就是桶排序了
        // 那么这个桶如何设置呢？

        // 方法2
        // 我们观察发现，所有的数字都是10进制组成的，那么我们很自然的就能放入10个位置
        // 这样我们分别将个位，10位，100位……依次放入桶，直到最高位那个数字已经放入桶
        // 最后将桶内的数据取出来就是有序的了
        // 然后直接搜索结果即可
        // 例如  322 23 12389 2
        // 第一轮，个位数 %10
        //  0  1  2    3  4  5  6  7  8  9
        //        322 23                 12389
        //        2
        // 322 2 23 12389
        // 第二轮，十位数 (x / 10) % 10
        //  0  1  2    3    4   5   6  7  8        9
        //  2     322                     12389
        //        23
        //  2  322 23  12389
        // 第三轮，百位数 (x/100) % 10
        //  0  1  2    3    4   5   6  7  8       9
        //  2          322
        //  23         12389
        // 2 23 322 12389
        // 一直到最后，我们就排好序了
        // 可以看出来，这种排序，是基于进制基数的，又称为基数排序
        // 那么时间复杂度是需要看数据的位数的，这里i32最大的数是 2_147_483_647
        // 也就是10位，而每次排序扫描n个数字，实际就是 10n，也可以近似为O(n)的线性时间
        // 总之，这就是基数排序的时间复杂度，位数为k时，时间就是O(kn)
        // Passed 0ms 2.1mb
        // if nums.len() < 2 { return 0; }
        // let mut nums = nums;
        // let mut base = 1;
        // let max_num = *nums.iter().max().unwrap();
        // while base <= max_num {
        //     let mut bucket = vec![Vec::new(); 10];
        //     for x in nums {
        //         bucket[((x / base) % 10) as usize].push(x);
        //     }
        //     base *= 10;
        //     nums = bucket.into_iter().flatten().collect();
        // }
        // nums.windows(2).map(|x| x[1] - x[0]).max().unwrap()

        // 方法3
        // 我们第二种设置桶的方法
        // 通过基数排序我们知道桶分成了基数个，其实还挺浪费的，因为每轮都有很多空的桶
        // 而且要分k次，如果有这样一种情况，就是我们确定好桶的数量，然后一次扫描就把
        // 所有的数字都分入桶中，而且不用管桶里面的数字之间的大小，只需要管不同桶的距离
        // 就好了，而距离最大的那个就是答案
        // 其实我们用(最大值 - 最小值） / n - 1的间距来分割数字就可以达到目的，
        // 现在我们来数学验证一下是不是，假设数组有序A1..An，那么a1就是最小值，An自然就是最大值，
        // 用反证法，如果(max - min) / (n - 1)导致相邻两个桶的距离都小于它的话，那么有
        // an - a1 = (an - An-1) + (An-1 - An-2) ... + (A2 - A1) < (max-min)/(n-1) + .. (max - min)/(n-1)
        // an - a1 < (n-1)* (max-min)/(n-1) = max - min
        // An - A1 < max - min，这显然是矛盾的
        // 所以只要我们用(max - min)/(n - 1)来放入数字，那么桶内的数字就是比桶间的距离小的，就达到我们的目的了
        // Passed 0ms 2.1mb
        if nums.len() < 2 { return 0; }
        let n = nums.len();
        let min = *nums.iter().min().unwrap() as usize;
        let max = *nums.iter().max().unwrap() as usize;
        let size = 1.max((max - min) / (n - 1));
        let bucket_size = (max - min) / size + 1;
        let mut bucket = vec![(-1, -1); bucket_size]; //(min, max)
        for n in nums {
            let idx = (n as usize - min) / size;
            if bucket[idx].0 == -1 {
                bucket[idx] = (n, n);
            } else {
                bucket[idx].0 = bucket[idx].0.min(n);
                bucket[idx].1 = bucket[idx].1.max(n);
            }
        }
        let mut answer = 0;
        let mut prev = -1;
        for i in 0..bucket_size {
            if bucket[i].0 != -1 {
                if prev != -1 {
                    answer = answer.max((bucket[i].0 - bucket[prev as usize].1) as i32);
                }
                prev = i as i32;
            }
        }
        answer
    }
}

#[test]
fn test() {
    assert_eq!(Solution::maximum_gap(vec![3, 9, 6, 1]), 3);
    assert_eq!(Solution::maximum_gap(vec![1, 10000000]), 9999999);
}