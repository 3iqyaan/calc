
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token{
    Number(f64),
    Operator(Operator),
    LParen,
    RParen,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator{
    UnaryMinus,
    Power,
    Multiply,
    Divide,
    Modulus,
    Add,
    Subtract,
}


pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' | '.' => {
                let mut number_str = String::new();
                while let Some(&digit) = chars.peek() {
                    if digit.is_numeric() || digit == '.' {
                        number_str.push(digit);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let number = number_str.parse::<f64>().unwrap();
                tokens.push(Token::Number(number));
            }
            '+' => {
                tokens.push(Token::Operator(Operator::Add));
                chars.next();
            }
            '-' => {
                // Check if it's a unary minus
                if tokens.is_empty() || matches!(tokens.last(), Some(Token::Operator(_)) | Some(Token::LParen)) {
                    tokens.push(Token::Operator(Operator::UnaryMinus));
                } else {
                    tokens.push(Token::Operator(Operator::Subtract));
                }
                chars.next();
            }
            '*' => {
                tokens.push(Token::Operator(Operator::Multiply));
                chars.next();
            }
            '/' => {
                tokens.push(Token::Operator(Operator::Divide));
                chars.next();
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            '^' => {
                tokens.push(Token::Operator(Operator::Power));
                chars.next();
            }
            '%' => {
                tokens.push(Token::Operator(Operator::Modulus));
                chars.next();
            }
            _ => {
                chars.next(); // Skip unrecognized characters
            }
        }
    }

    tokens
}