use std::collections::HashMap;

fn fib(n: u128, cache: &mut HashMap<u128, u128>) -> u128 {
    if n < 2 {
        return n;
    } else {
        if cache.contains_key(&n) {
            return *cache.get(&n).unwrap();
        } else {
            let result = fib(n - 1, cache) + fib(n - 2, cache);
            cache.insert(n, result);
            return result;
        }
    }
}

fn main() {
    let mut cache = HashMap::new();
    dbg!(fib(100, &mut cache));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let mut cache = HashMap::new();
        let mut fib_100 = vec![];
        for i in 0..100 {
            fib_100.push(fib(i, &mut cache));
        }

        for i in 2..100 {
            assert!(fib_100[i] == fib_100[i - 1] + fib_100[i - 2]);
        }
    }
}
