mod q67 {
    pub fn add_binary(mut a: String, mut b: String) -> String {
        let mut carry = 0;
        let mut res = String::new();
        loop {
            let (n1, n2) = (a.pop(), b.pop());

            if n1 == None && n2 == None { break; }

            if let Some(x) = n1 {
                carry += x.to_digit(2).unwrap();
            }
            if let Some(x) = n2 {
                carry += x.to_digit(2).unwrap();
            }
            res.insert_str(0, &(carry % 2).to_string());
            carry = carry / 2;
        }
        if carry > 0 { res.insert_str(0, &carry.to_string()); }
        res
    }
}