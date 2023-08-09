fn sum_for(array: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for e in array {
        sum += e;
    }
    sum
}

fn sum_while(array: &Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut i = 0;
    while i != array.len() {
        sum += array[i];
        i += 1;
    }
    sum
}

fn sum_rec(array: &Vec<i32>, n: usize) -> i32 {
    if n == 0 {
        return 0;
    } else {
        return array[n - 1] + sum_rec(array, n - 1);
    }
}

fn main() {
    let arr = vec![1, 2, 3, 4, 5];
    dbg!(sum_rec(&arr, arr.len()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty() {
        let arr = vec![];
        assert_eq!(0, sum_for(&arr));
        assert_eq!(0, sum_while(&arr));
        assert_eq!(0, sum_rec(&arr, arr.len()));
    }

    #[test]
    fn test_positive() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(15, sum_for(&arr));
        assert_eq!(15, sum_while(&arr));
        assert_eq!(15, sum_rec(&arr, arr.len()));
    }

    #[test]
    fn test_negative() {
        let arr = vec![-1, -2, -3, -4, -5];
        assert_eq!(-15, sum_for(&arr));
        assert_eq!(-15, sum_while(&arr));
        assert_eq!(-15, sum_rec(&arr, arr.len()));
    }
}
