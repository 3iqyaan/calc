// Write tests for the calculator. 

#[cfg(test)]
mod tests {
    use crate::tokenize::tokenize;
    use crate::compute::compute;
    use crate::errors::Error;

    #[test]
    fn addition() {
        let input = "2 + 3";
        let tokens = tokenize(input);
        let result = compute(tokens).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn power_and_mod() {
        let input = "2 ^ 3 % 3";
        let tokens = tokenize(input);
        let result = compute(tokens).unwrap();
        assert_eq!(result, 2.0); // (2^3) % 3 = 8 % 3 = 2
    }

    #[test]
    fn op_precedence() {
        let input = "2 + 3 * 4";
        let tokens = tokenize(input);
        let result = compute(tokens).unwrap();
        assert_eq!(result, 14.0); // 2 + (3 * 4)
    }

    #[test]
    fn paren() {
        let input = "(2 + 3) * 4";
        let tokens = tokenize(input);
        let result = compute(tokens).unwrap();
        assert_eq!(result, 20.0); // (2 + 3) * 4
    }

    #[test]
    fn division_by_zero() {
        let input = "10 / 0";
        let tokens = tokenize(input);
        let result = compute(tokens);
        assert!(matches!(result, Err(Error::DivisionByZero)));
    }

    #[test]
    fn insufficient_ops() {
        let input = "5 +";
        let tokens = tokenize(input);
        let result = compute(tokens);
        assert!(matches!(result, Err(Error::InsufficientOperands)));
    }

    #[test]
    fn mismatched_paren() {
        let input = "(2 + 3 * 4";
        let tokens = tokenize(input);
        let result = compute(tokens);
        assert!(matches!(result, Err(Error::MismatchedParentheses)));
    }

    #[test]
    fn associativity() {
        let input = "2 ^ 3 ^ 2";
        let tokens = tokenize(input);
        let result = compute(tokens).unwrap();
        assert_eq!(result, 512.0); // 2 ^ (3 ^ 2) = 2 ^ 9 = 512
    }

    #[test]
    fn complex_expr() {
        let input = "3 + 5 * (2 ^ 3) - 4 / 2";
        let tokens = tokenize(input);
        let result = compute(tokens).unwrap();
        assert_eq!(result, 41.0); // 3 + 5 * (8) - 2 = 3 + 40 - 2 = 41.
    }

    #[test]
    fn floating_ops() {
        let input = "5.5 * 2.0 + 3.3";
        let tokens = tokenize(input);
        let result = compute(tokens).unwrap();
        assert_eq!(result, 14.3); // 5.5 * 2.0 + 3.3 = 11.0 + 3.3 = 14.3
    }

    #[test]
    fn negative() {
        let input = "-2 + 3";
        let tokens = tokenize(input);
        let result = compute(tokens).unwrap();
        assert_eq!(result, 1.0); // -2 + 3 = 1.0
        let input2 = "4 * -2";
        let tokens2 = tokenize(input2);
        let result2 = compute(tokens2).unwrap();
        assert_eq!(result2, -8.0); // 4 * -2 = -8.0
    }

    #[test]
    fn multiple_ops() {
        let input = "2 + 3 - 1 * 5 / (2 ^ 2) % 3";
        let tokens = tokenize(input);
        let result = compute(tokens).unwrap();
        assert_eq!(result, 3.75); // 2 + 3 - 1 * 5 / 4 % 3 = 2 + 3 - 5 / 4 = 5 - 1.25 = 3.75
    }

    #[test]
    fn zero_power() {
        let input = "0 ^ 0";
        let tokens = tokenize(input);
        let result = compute(tokens).unwrap();
        assert_eq!(result, 1.0); // 0^0 is generally considered as 1
    }

    #[test]
    fn negative_power() {
        let input = "2 ^ -3";
        let tokens = tokenize(input);
        let result = compute(tokens).unwrap();
        assert_eq!(result, 0.125); // 2^-3 = 1/8 = 0.125
    }
}