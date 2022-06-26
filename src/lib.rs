use crate::ExpressionEntry::Operand;
use crate::ExpressionEntry::Operator;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::notation_converter::infix_to_postfix;

const PLUS: &str = "+";
const MINUS: &str = "-";
const DIVIDE: &str = "/";
const MULTIPLY: &str = "*";

mod notation_converter;

enum ExpressionEntry {
    Operand(i32),
    Operator(&'static str),
}

pub struct PostfixNotation {
    expression: Vec<ExpressionEntry>,
    postfix_str: String,
}

impl Display for PostfixNotation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Expression in postfix notation {}", self.postfix_str)
    }
}

impl PostfixNotation {
    pub fn from_infix_string(infix: &str) -> Self {
        let postfix_str = infix_to_postfix(infix);
        let expression = postfix_to_expression(&postfix_str);
        PostfixNotation {
            expression,
            postfix_str,
        }
    }

    pub fn calculate(&self) -> i32 {
        let mut stack = Vec::<ExpressionEntry>::new();
        for expr in &self.expression {
            match expr {
                Operand(value) => stack.push(Operand(*value)),
                Operator(PLUS) => {
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
    if postfix.len() == 0 {
        return Vec::<ExpressionEntry>::new();
    }
    let expression = postfix
        .split(' ')
        .map(|item| match item {
            "+" => Operator(PLUS),
            "-" => Operator(MINUS),
            "/" => Operator(DIVIDE),
            "*" => Operator(MULTIPLY),
            _ => Operand(FromStr::from_str(item).unwrap()),
        })
        .collect();
    expression
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

    impl PostfixNotation {
        // Should be used for unit testing purposes only
        fn from_expression(expression: Vec<ExpressionEntry>) -> Self {
            let postfix_str = "".to_string();
            PostfixNotation {
                expression,
                postfix_str,
            }
        }
    }
    #[test]
    fn postfix_to_expression_valid_expression() {
        let expression = postfix_to_expression("13 5 + 2 *");

        let item1 = match expression[0] {
            Operand(v) => v,
            _ => panic!("Wrong type of expression item"),
        };
        let item2 = match expression[1] {
            Operand(v) => v,
            _ => panic!("Wrong type of expression item"),
        };
        let item3 = match expression[2] {
            Operator(v) => v,
            _ => panic!("Wrong type of expression item"),
        };
        let item4 = match expression[3] {
            Operand(v) => v,
            _ => panic!("Wrong type of expression item"),
        };
        let item5 = match expression[4] {
            Operator(v) => v,
            _ => panic!("Wrong type of expression item"),
        };
        assert_eq!(5, expression.len());
        assert_eq!(13, item1);
        assert_eq!(5, item2);
        assert_eq!(PLUS, item3);
        assert_eq!(2, item4);
        assert_eq!(MULTIPLY, item5);
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
