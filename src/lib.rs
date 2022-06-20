use crate::ExpressionEntry::Operand;
use crate::ExpressionEntry::Operator;
use std::env;
use std::error::Error;
use std::fs;

enum ExpressionEntry {
    Operand(i32),
    Operator(&'static str),
}

fn calculate(expression: Vec<ExpressionEntry>) -> i32 {
    let mut stack = Vec::<ExpressionEntry>::new();
    for expr in expression {
        match expr {
            Operand(_) => stack.push(expr),
            Operator("+") => {
                let mut operand1 = get_operand_from_stack(&mut stack);
                let mut operand2 = get_operand_from_stack(&mut stack);
                stack.push(Operand(operand2 + operand1))
            }
            Operator("-") => {
                let mut operand1 = get_operand_from_stack(&mut stack);
                let mut operand2 = get_operand_from_stack(&mut stack);
                stack.push(Operand(operand2 - operand1))
            }
            Operator("*") => {
                let mut operand1 = get_operand_from_stack(&mut stack);
                let mut operand2 = get_operand_from_stack(&mut stack);
                stack.push(Operand(operand2 * operand1))
            }
            Operator("/") => {
                let mut operand1 = get_operand_from_stack(&mut stack);
                let mut operand2 = get_operand_from_stack(&mut stack);
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
    let mut result = 0;
    if let Some(Operand(value)) = stack.pop() {
        result = value
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
    fn sum_values_return_their_sum() {
        let expression = vec![Operand(4), Operand(13), Operator("+")];

        assert_eq!(17, calculate(expression));
    }

    #[test]
    fn subtract_values_return_their_subtraction() {
        let expression = vec![Operand(4), Operand(13), Operator("-")];

        assert_eq!(-9, calculate(expression));
    }

    #[test]
    fn multiply_values_return_their_product() {
        let expression = vec![Operand(4), Operand(13), Operator("*")];

        assert_eq!(52, calculate(expression));
    }

    #[test]
    fn divide_values_return_their_division() {
        let expression = vec![Operand(13), Operand(4), Operator("/")];

        assert_eq!(3, calculate(expression));
    }

    #[test]
    #[should_panic]
    fn unknown_operator_leads_to_panic() {
        let expression = vec![Operand(13), Operand(4), Operator("&")];
        calculate(expression);
    }

    #[test]
    fn complex_expression_evaluation_returns_result() {
        let expression = vec![
            Operand(4),
            Operand(13),
            Operand(5),
            Operator("/"),
            Operator("+"),
        ];
        assert_eq!(6, calculate(expression));
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
