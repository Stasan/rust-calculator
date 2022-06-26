use crate::ExpressionEntry::Operand;
use crate::ExpressionEntry::Operator;
// use std::env;
// use std::error::Error;
// use std::fs;

enum ExpressionEntry {
    Operand(i32),
    Operator(&'static str),
}

pub struct PostfixNotation {
    expression: Vec<ExpressionEntry>,
}

impl PostfixNotation {
    fn from_expression(expression: Vec<ExpressionEntry>) -> Self {
        PostfixNotation { expression }
    }

    fn from_infix_string(_infix: &str) -> Self {
        let expression = Vec::<ExpressionEntry>::new();
        // let mut postfix

        PostfixNotation { expression }
    }

    pub fn calculate(&self) -> i32 {
        let mut stack = Vec::<ExpressionEntry>::new();
        for expr in &self.expression {
            match expr {
                Operand(value) => stack.push(Operand(*value)),
                Operator("+") => {
                    let operand1 = get_operand_from_stack(&mut stack);
                    let operand2 = get_operand_from_stack(&mut stack);
                    stack.push(Operand(operand2 + operand1))
                }
                Operator("-") => {
                    let operand1 = get_operand_from_stack(&mut stack);
                    let operand2 = get_operand_from_stack(&mut stack);
                    stack.push(Operand(operand2 - operand1))
                }
                Operator("*") => {
                    let operand1 = get_operand_from_stack(&mut stack);
                    let operand2 = get_operand_from_stack(&mut stack);
                    stack.push(Operand(operand2 * operand1))
                }
                Operator("/") => {
                    let operand1 = get_operand_from_stack(&mut stack);
                    let operand2 = get_operand_from_stack(&mut stack);
                    stack.push(Operand(operand2 / operand1))
                }
                Operator(value) => {
                    panic!(
                        "Got unknown Operator {} instead of valid operator. Check your expression.",
                        value
                    )
                }
            };
        }
        get_operand_from_stack(&mut stack)
    }
}

fn postfix_to_expression(postfix: &str) -> Vec<ExpressionEntry> {
    let mut expression = Vec::<ExpressionEntry>::new();

    expression
}

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
            '0'..='9' => {
                result.push(c)},
            '(' => stack.push(c),
            ')' => {
                while stack.len() > 0 && *stack.last().unwrap() != '(' {
                    println!("Cycle 1");
                    result.push(' ');
                    result.push(stack.pop().unwrap());
                }

                println!("{}", stack.len());
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
        let operator:char = stack.pop().unwrap();
        if operator == '('
        {
            panic!("Brackets mismatch. Check your expression.");
        }
        result.push(' ');
        result.push(operator);
    }
    result
}

fn get_operand_from_stack(stack: &mut Vec<ExpressionEntry>) -> i32 {
    let operand = stack.pop();
    match operand {
        None => {
            panic!("Got None instead of valid operand. Check your expression.")
        }
        Some(value) => match value {
            Operand(value) => {
                return value;
            }
            Operator(_) => {
                panic!("Got Operator instead of valid operand. Check your expression.")
            }
        },
    }
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
        print!("{}", infix_to_postfix("13+5"));
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

    #[test]
    fn from_empty_infix_string_empty_instance_created() {
        let notation = PostfixNotation::from_infix_string("");
        assert_eq!(0, notation.expression.len());
    }

    #[test]
    fn sum_values_return_their_sum() {
        let expression = vec![Operand(4), Operand(13), Operator("+")];

        let notation = PostfixNotation::from_expression(expression);
        assert_eq!(17, notation.calculate());
    }

    #[test]
    fn subtract_values_return_their_subtraction() {
        let expression = vec![Operand(4), Operand(13), Operator("-")];
        let notation = PostfixNotation::from_expression(expression);
        assert_eq!(-9, notation.calculate());
    }

    #[test]
    fn multiply_values_return_their_product() {
        let expression = vec![Operand(4), Operand(13), Operator("*")];
        let notation = PostfixNotation::from_expression(expression);
        assert_eq!(52, notation.calculate());
    }

    #[test]
    fn divide_values_return_their_division() {
        let expression = vec![Operand(13), Operand(4), Operator("/")];
        let notation = PostfixNotation::from_expression(expression);
        assert_eq!(3, notation.calculate());
    }

    #[test]
    #[should_panic]
    fn unknown_operator_leads_to_panic() {
        let expression = vec![Operand(13), Operand(4), Operator("&")];
        let notation = PostfixNotation::from_expression(expression);
        notation.calculate();
    }

    #[test]
    fn complex_expression_evaluation_returns_correct_result() {
        let expression = vec![
            Operand(4),
            Operand(13),
            Operand(5),
            Operator("/"),
            Operator("+"),
        ];
        let notation = PostfixNotation::from_expression(expression);
        assert_eq!(6, notation.calculate());
    }

    #[test]
    fn get_operand_from_stack_return_valid_value() {
        let mut stack = vec![Operand(4), Operand(13)];
        assert_eq!(13, get_operand_from_stack(&mut stack));
        assert_eq!(4, get_operand_from_stack(&mut stack));
    }

    #[test]
    #[should_panic]
    fn get_operand_from_stack_empty_stack_lead_to_panic() {
        let mut stack = Vec::<ExpressionEntry>::new();
        get_operand_from_stack(&mut stack);
    }

    #[test]
    #[should_panic]
    fn get_operand_from_stack_operator_on_head_lead_to_panic() {
        let mut stack = vec![Operator("+")];
        get_operand_from_stack(&mut stack);
    }
}
