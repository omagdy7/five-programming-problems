use std::collections::{HashSet, VecDeque};

fn evalute_expression(expr: &mut String) -> i64 {
    let mut ans = 0;
    let mut stack_op: VecDeque<char> = VecDeque::new();
    expr.push('+');
    expr.push('0');

    let mut stack_num: VecDeque<String> = expr
        .split(|c| c == '+' || c == '-')
        .map(String::from)
        .collect();
    for e in expr.chars() {
        if e == '+' || e == '-' {
            stack_op.push_back(e);
        }
    }

    while !stack_op.is_empty() {
        match stack_op.pop_front().unwrap() {
            '+' => {
                let left = stack_num.pop_front().unwrap().parse::<i64>().unwrap();
                let right = stack_num.pop_front().unwrap().parse::<i64>().unwrap();

                ans = left + right;
                stack_num.push_front(ans.to_string());
            }
            '-' => {
                let left = stack_num.pop_front().unwrap().parse::<i64>().unwrap();
                let right = stack_num.pop_front().unwrap().parse::<i64>().unwrap();

                ans = left - right;

                stack_num.push_front(ans.to_string());
            }
            _ => {}
        }
    }
    ans
}

fn possible_combs_sum_to_target(
    target: i64,
    num: u32,
    op: &str,
    mut expression: String,
    set: &mut HashSet<String>,
) {
    if num == 10 {
        if ['+', '-'].contains(&expression.chars().last().unwrap()) {
            expression.pop();
        }
        if evalute_expression(&mut expression) == target {
            expression.pop();
            expression.pop();
            set.insert(expression);
        }

        return;
    }
    for str in &["+", "-", ""] {
        let new_str = expression.clone() + format!("{num}{str}").as_str();
        possible_combs_sum_to_target(target, num + 1, str, new_str, set);
    }
}

fn main() {
    let mut set: HashSet<String> = HashSet::new();
    possible_combs_sum_to_target(42, 1, "", String::new(), &mut set);
    for expr in set {
        println!("{expr} = 42");
    }
}
