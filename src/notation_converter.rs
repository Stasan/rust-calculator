fn operator_priority(operator: &char) -> i32 {
    match *operator {
        '*' => 2,
        '/' => 2,
        '-' => 1,
        '+' => 1,
        _ => -1,
    }
}

pub fn infix_to_postfix(infix: &str) -> String {
    let mut result: String = "".to_owned();

    let mut stack = Vec::<char>::new();

    for c in infix.chars() {
        match c {
            '0'..='9' => result.push(c),
            '(' => stack.push(c),
            ')' => {
                while stack.len() > 0 && *stack.last().unwrap() != '(' {
                    result.push(' ');
                    result.push(stack.pop().unwrap());
                }

                if stack.len() == 0 || (stack.len() > 0 && *stack.last().unwrap() != '(') {
                    panic!("Brackets mismatch. Check your expression.")
                } else {
                    stack.pop();
                }
            }

            _ => {
                result.push(' ');
                while stack.len() > 0
                    && operator_priority(&c) <= operator_priority(stack.last().unwrap())
                {
                    result.push(stack.pop().unwrap());
                    result.push(' ');
                }
                stack.push(c);
            }
        }
    }

    while stack.len() > 0 {
        let operator: char = stack.pop().unwrap();
        if operator == '(' {
            panic!("Brackets mismatch. Check your expression.");
        }
        result.push(' ');
        result.push(operator);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::ExpressionEntry::*;
    use super::*;

    #[test]
    fn from_empty_infix_string_empty_postfix_string() {
        assert_eq!("", infix_to_postfix(""));
    }

    #[test]
    fn infix_to_postfix_add() {
        assert_eq!("13 5 +", infix_to_postfix("13+5"));
    }

    #[test]
    fn infix_to_postfix_subtract() {
        assert_eq!("13 5 -", infix_to_postfix("13-5"));
    }

    #[test]
    fn infix_to_postfix_divide() {
        assert_eq!("13 5 /", infix_to_postfix("13/5"));
    }

    #[test]
    fn infix_to_postfix_multiply() {
        assert_eq!("13 5 *", infix_to_postfix("13*5"));
    }

    #[test]
    fn infix_to_postfix_multiplication_higher_priority_than_add() {
        assert_eq!("13 5 2 * +", infix_to_postfix("13+5*2"));
    }

    #[test]
    fn infix_to_postfix_multiplication_higher_priority_than_subtract() {
        assert_eq!("13 5 2 * -", infix_to_postfix("13-5*2"));
    }

    #[test]
    fn infix_to_postfix_division_higher_priority_than_add() {
        assert_eq!("13 5 2 / +", infix_to_postfix("13+5/2"));
    }

    #[test]
    fn infix_to_postfix_division_higher_priority_than_subtract() {
        assert_eq!("13 5 2 / -", infix_to_postfix("13-5/2"));
    }

    #[test]
    fn test() {
        assert_eq!("", infix_to_postfix(""));
    }

    #[test]
    fn infix_to_postfix_brackets_change_priority() {
        assert_eq!("13 5 + 2 *", infix_to_postfix("(13+5)*2"));
    }

    #[test]
    #[should_panic]
    fn infix_to_postfix_no_open_bracket() {
        infix_to_postfix("13+5)*2");
    }

    #[test]
    #[should_panic]
    fn infix_to_postfix_no_close_bracket() {
        infix_to_postfix("(13+5*2");
    }
}
