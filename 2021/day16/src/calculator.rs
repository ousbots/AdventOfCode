use crate::parser::Token;

// Calculate the value of the tokens based on the following operations.
// Token value -> operation
//  0 -> sum of subvalues
//  1 -> product of subvalues
//  2 -> minimum of subvalues
//  3 -> maximum of subvalues
//  4 -> value
//  5 -> first value > second value
//  6 -> first value < second value
//  7 -> first value = second value
pub fn calculate(tokens: &mut Vec<Token>) -> Option<i64> {
    let token = tokens.pop()?;
    match token.class {
        // Sum of values.
        0 => {
            let mut values = Vec::<i64>::new();

            while tokens.len() > 0 {
                values.push(calculate(tokens).unwrap());
            }
            let result: i64 = values.iter().sum();

            println!("add {:?}", values);
            println!("result {}", result);

            Some(result)
        }

        // Product of values.
        1 => {
            let mut values = Vec::<i64>::new();

            while tokens.len() > 0 {
                values.push(calculate(tokens).unwrap());
            }
            let result: i64 = values.iter().fold(1, |acc, x| acc * x);

            println!("mult {:?}", values);
            println!("result {}", result);

            Some(result)
        }

        // Minimum of values.
        2 => {
            let mut values = Vec::<i64>::new();

            while tokens.len() > 0 {
                values.push(calculate(tokens).unwrap());
            }
            values.sort();
            let result = *values.get(0).unwrap();

            println!("min {:?}", values);
            println!("result {}", result);

            Some(result)
        }

        // Maximum of values.
        3 => {
            let mut values = Vec::<i64>::new();

            while tokens.len() > 0 {
                values.push(calculate(tokens).unwrap());
            }
            values.sort();
            let result = *values.get(values.len() - 1).unwrap();

            println!("max {:?}", values);
            println!("result {}", result);

            Some(result)
        }

        4 => {
            println!("val {}", token.value);
            Some(token.value)
        }

        // First value greater than second.
        5 => {
            let first = calculate(tokens).unwrap();
            let second = calculate(tokens).unwrap();

            let result = if first > second { 1 } else { 0 };

            println!("{} > {}", first, second);
            println!("result {}", result);

            Some(result)
        }

        // First value less than second.
        6 => {
            let first = calculate(tokens).unwrap();
            println!("first {}", first);
            let second = calculate(tokens).unwrap();
            let result = if first < second { 1 } else { 0 };

            println!("{} < {}", first, second);
            println!("result {}", result);

            Some(result)
        }

        // First value equal to second.
        7 => {
            let first = calculate(tokens).unwrap();
            let second = calculate(tokens).unwrap();
            let result = if first == second { 1 } else { 0 };

            println!("{} == {}", first, second);
            println!("result {}", result);

            Some(result)
        }

        _ => panic!("invalid type ID {}", token.class),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn token(class: i64, value: i64) -> Token {
        Token {
            version: 0,
            class: class,
            value: value,
        }
    }

    #[test]
    fn basic_functions() {
        // + 3 = 3
        let mut chain = vec![token(0, 0), token(4, 3)];
        chain.reverse();
        assert_eq!(calculate(&mut chain).unwrap(), 3);

        // + 1 3 = 4
        let mut chain = vec![token(0, 0), token(4, 1), token(4, 3)];
        chain.reverse();
        assert_eq!(calculate(&mut chain).unwrap(), 4);

        // * 2 = 2
        let mut chain = vec![token(1, 0), token(4, 2)];
        chain.reverse();
        assert_eq!(calculate(&mut chain).unwrap(), 2);

        // * 1 3 = 3
        let mut chain = vec![token(1, 0), token(4, 1), token(4, 3)];
        chain.reverse();
        assert_eq!(calculate(&mut chain).unwrap(), 3);

        // min 1 2 3 = 1
        let mut chain = vec![token(2, 0), token(4, 1), token(4, 2), token(4, 3)];
        chain.reverse();
        assert_eq!(calculate(&mut chain).unwrap(), 1);

        // min 2 = 2
        let mut chain = vec![token(1, 0), token(4, 2)];
        chain.reverse();
        assert_eq!(calculate(&mut chain).unwrap(), 2);

        // max 1 2 3 = 3
        let mut chain = vec![token(3, 0), token(4, 1), token(4, 2), token(4, 3)];
        chain.reverse();
        assert_eq!(calculate(&mut chain).unwrap(), 3);

        // max 2 = 2
        let mut chain = vec![token(3, 0), token(4, 2)];
        chain.reverse();
        assert_eq!(calculate(&mut chain).unwrap(), 2);

        // val 6 = 6
        let mut chain = vec![token(4, 6)];
        chain.reverse();
        assert_eq!(calculate(&mut chain).unwrap(), 6);

        // > 1 2 = 0
        let mut chain = vec![token(5, 0), token(4, 1), token(4, 2)];
        chain.reverse();
        assert_eq!(calculate(&mut chain).unwrap(), 0);

        // > 2 1 = 1
        let mut chain = vec![token(5, 0), token(4, 2), token(4, 1)];
        chain.reverse();
        assert_eq!(calculate(&mut chain).unwrap(), 1);

        // < 1 2 = 1
        let mut chain = vec![token(6, 0), token(4, 1), token(4, 2)];
        chain.reverse();
        assert_eq!(calculate(&mut chain).unwrap(), 1);

        // < 2 1 = 0
        let mut chain = vec![token(6, 0), token(4, 2), token(4, 1)];
        chain.reverse();
        assert_eq!(calculate(&mut chain).unwrap(), 0);

        // = 1 2 = 0
        let mut chain = vec![token(7, 0), token(4, 1), token(4, 2)];
        chain.reverse();
        assert_eq!(calculate(&mut chain).unwrap(), 0);

        // = 2 2 = 1
        let mut chain = vec![token(7, 0), token(4, 2), token(4, 2)];
        chain.reverse();
        assert_eq!(calculate(&mut chain).unwrap(), 1);
    }

    #[test]
    fn basic_chain() {
        // * + 1 2 3 = 9
        let mut chain = vec![
            token(1, 0),
            token(0, 0),
            token(4, 1),
            token(4, 2),
            token(4, 3),
        ];
        chain.reverse();
        assert_eq!(calculate(&mut chain).unwrap(), 9);

        // > * + 1 2 3 1 = 1
        let mut chain = vec![
            token(5, 0),
            token(1, 0),
            token(0, 0),
            token(4, 1),
            token(4, 2),
            token(4, 3),
            token(4, 1),
        ];
        chain.reverse();
        assert_eq!(calculate(&mut chain).unwrap(), 1);

        // < * + 1 2 2 1 = 0
        let mut chain = vec![
            token(6, 0),
            token(1, 0),
            token(0, 0),
            token(4, 1),
            token(4, 2),
            token(4, 2),
            token(4, 1),
        ];
        chain.reverse();
        assert_eq!(calculate(&mut chain).unwrap(), 0);

        // max * + 1 2 2 7 8 = 8
        let mut chain = vec![
            token(3, 0),
            token(1, 0),
            token(0, 0),
            token(4, 1),
            token(4, 2),
            token(4, 2),
            token(4, 7),
            token(4, 8),
        ];
        chain.reverse();
        assert_eq!(calculate(&mut chain).unwrap(), 8);

        // min * + 1 2 2 7 8 = 6
        let mut chain = vec![
            token(2, 0),
            token(1, 0),
            token(0, 0),
            token(4, 1),
            token(4, 2),
            token(4, 2),
            token(4, 7),
            token(4, 8),
        ];
        chain.reverse();
        assert_eq!(calculate(&mut chain).unwrap(), 6);
    }

    #[test]
    fn nested_chains() {
        // max * + 1 2 2 5 * > 2 1 8  = 8
        let mut chain = vec![
            token(3, 0),
            token(1, 0),
            token(0, 0),
            token(4, 1),
            token(4, 2),
            token(4, 2),
            token(4, 5),
            token(1, 0),
            token(5, 0),
            token(4, 2),
            token(4, 1),
            token(4, 8),
        ];
        chain.reverse();
        assert_eq!(calculate(&mut chain).unwrap(), 8);
    }
}
