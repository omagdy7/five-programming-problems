use std::cmp::Ordering;

fn custom_sort(a: i32, b: i32) -> Ordering {
    use std::cmp::max;
    let mut str_a = a.to_string();
    let mut str_b = b.to_string();

    let mut cnt = 0;
    let len_a = str_a.len();
    let len_b = str_b.len();

    if len_a > len_b {
        str_b.extend("0".repeat(len_a - len_b).chars());
    } else {
        str_a.extend("0".repeat(len_b - len_a).chars());
    }

    let len = max(len_a, len_b);

    println!("comparing {a} and {b}");

    while cnt < len && str_a.chars().nth(cnt) == str_b.chars().nth(cnt) {
        cnt += 1;
    }

    println!("cnt is {cnt}");

    if str_a.chars().nth(cnt) == str_b.chars().nth(cnt) {
        return Ordering::Equal;
    } else if str_a.chars().nth(cnt) > str_b.chars().nth(cnt) {
        return Ordering::Greater;
    } else {
        return Ordering::Less;
    }
}

fn largest_possible_num(arr: &Vec<i32>) -> String {
    let mut ans = String::new();
    if arr.iter().sum::<i32>() == 0 {
        return String::from("0");
    }
    for e in arr {
        ans += e.to_string().as_str();
    }
    ans
}

fn main() {
    let mut arr = vec![8, 72, 71, 70, 5, 81];
    arr.sort_by(|&a, &b| custom_sort(b, a));
    dbg!(largest_possible_num(&arr));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one() {
        // Test cases
        let mut tests = [
            (vec![9, 50, 2, 1], "95021"),
            (vec![3, 30, 34, 5, 9], "9534330"),
            (vec![0, 0, 0, 0], "0"),
            (vec![10, 2], "210"),
            (vec![432, 43243], "43243432"),
            (
                vec![824, 938, 1399, 5607, 6971, 5703],
                "9388246971570356071399",
            ),
            (vec![1, 22, 33, 444, 5555], "555544433221"),
            (vec![987, 97, 965], "98797965"),
            (vec![100, 200, 300], "300200100"),
            (vec![5, 55, 555], "555555"),
            (vec![4321, 4322, 4324], "432443224321"),
            (vec![998, 996, 994], "998996994"),
            (vec![33, 331, 34], "3433133"),
            (vec![0, 5, 9, 8], "9850"),
            (vec![11, 12, 1], "12111"),
        ];

        for (input, expected) in &mut tests {
            input.sort_by(|&a, &b| custom_sort(b, a));
            let result = largest_possible_num(input);
            assert_eq!(&result, *expected);
        }
    }
}
