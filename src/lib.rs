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
                let (operand1, operand2) = get_operands_values(&mut stack);
                stack.push(Operand(operand1 + operand2))
            }
            Operator("-") => {
                let (operand1, operand2) = get_operands_values(&mut stack);
                stack.push(Operand(operand2 - operand1))
            }
            Operator("*") => {
                let (operand1, operand2) = get_operands_values(&mut stack);
                stack.push(Operand(operand2 * operand1))
            }
            Operator("/") => {
                let (operand1, operand2) = get_operands_values(&mut stack);
                stack.push(Operand(operand2 / operand1))
            }
            _ => {}
        };
    }
    let mut result = 0;
    if let Some(Operand(value)) = stack.pop() {
        result = value
    }
    result
}

fn get_operands_values(stack: &mut Vec<ExpressionEntry>) -> (i32, i32) {
    let mut operand1 = 0;
    let mut operand2 = 0;
    if let Some(ExpressionEntry::Operand(value)) = stack.pop() {
        operand1 = value;
    }
    if let Some(ExpressionEntry::Operand(value)) = stack.pop() {
        operand2 = value;
    }
    (operand1, operand2)
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
    fn complex_expression_evaluation_returns_result() {
        // ["4", "13", "5", "/", "+"]
        let expression = vec![Operand(4), Operand(13), Operand(5), Operator("/"), Operator("+")];
        assert_eq!(6, calculate(expression));
    }
}
