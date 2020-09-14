use crate::q::Solution;

impl Solution {
    pub fn largest_time_from_digits(a: Vec<i32>) -> String {
        // solution 2
        let mut max = (0, vec![]);
        for i in 0..4 {
            for j in 0..4 {
                if i == j { continue; }
                for k in 0..4 {
                    if i == k || j == k { continue; }
                    for l in 0..4 {
                        if i == l || j == l || k == l { continue; }
                        let hour = a[i] * 10 + a[j];
                        let minute = a[k] * 10 + a[l];
                        if hour < 24 && minute < 60 {
                            if hour * 60 + minute >= max.0 {
                                max.0 = hour * 60 + minute;
                                max.1 = vec![a[i], a[j], a[k], a[l]];
                            }
                        }
                    }
                }
            }
        }
        if max.1.len() == 0 { return String::new(); }
        format!("{}{}:{}{}", max.1[0], max.1[1], max.1[2], max.1[3])


        // solution 1

        // let mut p = Vec::new();
        // for i in 0..a.len() {
        //     if a[i] <= 2 {
        //         p.push(i);
        //     }
        // }
        // let mut max = (0, vec![]);

        // for i in 0..p.len() {
        //     let mut a = a.clone();
        //     let mut time = vec![-1; 4];
        //     time[0] = a[p[i]];
        //     a.remove(p[i]);
        //
        //     let mut r = None;
        //     for j in 0..a.len() {
        //         let mut max = 3;
        //         if time[0] < 2 { max = 9; }
        //         if a[j] <= max && a[j] > time[1] {
        //             time[1] = a[j];
        //             r = Some(j);
        //         }
        //     }
        //     if r == None { continue; } else { a.remove(r.unwrap()); }
        //     r = None;
        //
        //     for j in 0..a.len() {
        //         if a[j] <= 5 && a[j] > time[2] {
        //             time[2] = a[j];
        //             r = Some(j);
        //         }
        //     }
        //     if r == None { continue; } else { a.remove(r.unwrap()); }
        //
        //     time[3] = a[0];
        //
        //     let sum = (time[0] * 10 + time[1]) * 60 + time[2] * 10 + time[3];
        //     if sum >= max.0 {
        //         max.0 = sum;
        //         max.1 = time;
        //     }
        // }
        //
        // if max.1.len() == 0 { return String::new(); }
        // format!("{}{}:{}{}", max.1[0], max.1[1], max.1[2], max.1[3])
    }
}

#[test]
fn test_q949() {
    let questions = vec![
        (vec![5, 5, 5, 5], ""),
        (vec![1, 2, 3, 4], "23:41"),
        (vec![0, 0, 0, 0], "00:00"),
        (vec![5, 9, 3, 4], ""),
        (vec![5, 2, 3, 9], "23:59"),
        (vec![0, 0, 0, 1], "10:00"),
        (vec![2, 0, 6, 6], "06:26"),// 06:26
        (vec![2, 0, 2, 6], "22:06"),// 22:06
        (vec![0, 6, 2, 6], "06:26"),// 06:26
        (vec![4, 1, 0, 0], "14:00"),// 14:00
    ];
    for q in questions {
        assert_eq!(q.1, &Solution::largest_time_from_digits(q.0));
    }
}