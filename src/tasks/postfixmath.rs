/// Evaluate a valid mathematical expression (in postfix notation)
pub fn evaluate_postfix(expr: &str) -> f64 {
    // we assume that input has been evaluated to be a
    // valid expression in the postfix notation
    let ops = ["+", "-", "*", "/", "^", "sqrt"];
    let mut stack = Vec::new();
    for tok in expr.split_whitespace() {
        if ops.contains(&tok) {
            let res = eval(tok, &mut stack);
            stack.push(res);
        } else {
            // evaluate to float and push on stack
            let x: f64 = tok.parse().unwrap();
            stack.push(x);
        }
    }
    stack.pop().unwrap()
}

fn eval(op: &str, stack: &mut Vec<f64>) -> f64 {
    if op == "sqrt" {
        let x = stack.pop().unwrap();
        x.sqrt()
    } else {
        let (y, x) = (stack.pop().unwrap(), stack.pop().unwrap());
        match op {
            "+" => x + y,
            "-" => x - y,
            "*" => x * y,
            "/" => x / y,
            "^" => x.powf(y),
            _ => f64::INFINITY, // non-branch, we are checking for valid ops
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(evaluate_postfix("1"), 1.0)
    }
    #[test]
    fn test_add_subtract() {
        assert_eq!(evaluate_postfix("5 6 + 7 -"), 4.0)
    }
    #[test]
    fn test_sqrt() {
        let abs_diff = (evaluate_postfix("2 3.14 sqrt *") - 3.54400902933387).abs();
        assert!(abs_diff < 1e-10)
    }
    #[test]
    fn test_pow_div() {
        assert_eq!(evaluate_postfix("8.82 2.1 2 ^ /"), 2.0)
    }
}
