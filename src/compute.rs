use crate::tokenize::{Token, Operator};
use crate::errors::{Error, Result};

/// Computes the result using the shunting-yard algorithm
pub fn eval(tokens: Vec<Token>) -> Result<f64> {

    let mut stack = Vec::new();

    for tok in tokens {
        match tok {
            Token::Number(n) => stack.push(n),

            Token::Operator(op) => {
                if op == Operator::UnaryMinus {
                    let a = stack.pop().ok_or(Error::InsufficientOperands)?;
                    stack.push(-a);
                    continue;
                }
                let b = stack.pop().ok_or(Error::InsufficientOperands)?;
                let a = stack.pop().ok_or(Error::InsufficientOperands)?;
                let res = match op {
                    Operator::Add => a + b,
                    Operator::Subtract => a - b,
                    Operator::Multiply => a * b,
                    Operator::Divide => a / b,
                    Operator::Modulus => a % b,
                    Operator::Power => a.powf(b),
                    Operator::UnaryMinus => unreachable!()
                };
                if res.is_infinite() || res.is_nan() {
                    return Err(Error::DivisionByZero);
                }
                stack.push(res);
            }
            Token::LParen | Token::RParen => {
                return Err(Error::MismatchedParentheses);
            }
        }
    }

    stack.pop().ok_or(Error::InsufficientOperands)
}

fn precedence(op: &Operator) -> u8 {
    match op {
        Operator::Add | Operator::Subtract => 10,
        Operator::Multiply | Operator::Divide | Operator::Modulus => 20,
        Operator::Power => 30,
        Operator::UnaryMinus => 40,
    }
}

fn is_right_associative(op: &Operator) -> bool {
    matches!(op, Operator::Power)
}

pub fn to_rpn(tokens: Vec<Token>) -> Vec<Token> {
    let mut output = Vec::new();
    let mut ops = Vec::new();

    for token in tokens {
        match token {
            Token::Number(_) => output.push(token),
            Token::Operator(op1) => {
                while let Some(Token::Operator(op2)) = ops.last() {
                    let prec1 = precedence(&op1);
                    let prec2 = precedence(op2);
                    if (is_right_associative(&op1) && prec1 < prec2) || (!is_right_associative(&op1) && prec1 <= prec2){
                        // unwrap is safe here because we just checked ops is not empty
                        output.push(ops.pop().unwrap());
                    }
                    else {
                        break;
                    }
                }
                ops.push(token);
            }
            Token::LParen => ops.push(token),
            Token::RParen => {
                while let Some(top) = ops.pop() {
                    if let Token::LParen = top {
                        break;
                    } else {
                        output.push(top);
                    }
                }
            }
        }
    }

    while let Some(op) = ops.pop() {
        output.push(op);
    }

    output
}
