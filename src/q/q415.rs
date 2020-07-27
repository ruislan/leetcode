mod q415 {
    pub fn add_strings(mut num1: String, mut num2: String) -> String {
        // 方法1
        // let mut res = String::new();
        // let mut carry = 0u32;
        // loop {
        //     let n1 = num1.pop();
        //     let n2 = num2.pop();
        //     if n1 == None && n2 == None { break; }
        //
        //     let mut sum = 0u32;
        //     if n1 != None {
        //         sum += n1.unwrap().to_digit(10).unwrap();
        //     }
        //     if n2 != None {
        //         sum += n2.unwrap().to_digit(10).unwrap();
        //     }
        //     sum += carry;
        //     if sum >= 10 {
        //         carry = 1;
        //         sum -= 10;
        //     } else { carry = 0; }
        //
        //     res.insert_str(0, &sum.to_string());
        // }
        // if carry > 0 {
        //     res.insert_str(0, &carry.to_string());
        // }
        // res
        
        // 方法2
        let mut res = String::new();
        let mut carry = 0u32;
        loop {
            if num1.is_empty() && num2.is_empty() && carry == 0 { break; }
            if let Some(n1) = num1.pop() { carry += n1.to_digit(10).unwrap(); }
            if let Some(n2) = num2.pop() { carry += n2.to_digit(10).unwrap(); }
            res.push_str(&(carry % 10).to_string());
            carry /= 10;
        }
        res.chars().rev().collect::<String>()
    }
}