mod q811 {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut map = std::collections::HashMap::new();
        for cpdomain in cpdomains {
            let mut iter = cpdomain.split_ascii_whitespace();
            let times = u32::from_str_radix(iter.next().unwrap(), 10).unwrap();
            let domain = iter.next().unwrap();
            let mut ds: Vec<&str> = domain.split('.').collect();
            let mut hs = String::new();
            while let Some(x) = ds.pop() {
                let mut host = hs.clone();
                if host.is_empty() { host.push_str(x); } else {
                    host.insert(0, '.');
                    host.insert_str(0, x);
                }
                let h = map.entry(host.clone()).or_insert(0);
                *h += times;
                hs = host.clone();
            }
        }
        let mut res = Vec::new();
        for (k, v) in map {
            let mut s = v.to_string();
            s.push(' ');
            s.push_str(&k);
            res.push(s);
        }
        res
    }
}