fn combine_arr(arr1: &Vec<i32>, arr2: &Vec<i32>) -> Vec<i32> {
    let mut ret = Vec::with_capacity(arr1.len() + arr2.len());
    for (&e1, &e2) in arr1.iter().zip(arr2) {
        ret.push(e1);
        ret.push(e2);
    }
    ret
}

fn main() {
    let arr1 = vec![0, 2, 4];
    let arr2 = vec![1, 3, 5];
    dbg!(combine_arr(&arr1, &arr2));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty() {
        let arr1 = vec![];
        let arr2 = vec![];
        let empty_vec: Vec<i32> = vec![];
        assert_eq!(empty_vec, combine_arr(&arr1, &arr2));
    }

    #[test]
    fn test_combine_arr() {
        let arr1 = vec![0, 2, 4];
        let arr2 = vec![1, 3, 5];
        assert_eq!(vec![0, 1, 2, 3, 4, 5], combine_arr(&arr1, &arr2));
    }
}
