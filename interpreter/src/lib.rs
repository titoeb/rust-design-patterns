pub struct Interpreter<'a> {
    simple_expression: std::str::Chars<'a>,
}

#[derive(Debug)]
pub enum InterpreterError {
    UnexpectedSymbol(char),
    UnextpectedEnd,
}

impl<'a> Interpreter<'a> {
    pub fn new(infix: &'a str) -> Self {
        Self {
            simple_expression: infix.chars(),
        }
    }

    fn next_char(&mut self) -> Option<char> {
        self.simple_expression.next()
    }

    pub fn interpret(&mut self) -> Result<String, InterpreterError> {
        let mut postfix = String::new();
        match self.next_digit() {
            Ok(character) => postfix.push(character),
            Err(error) => return Err(error),
        }

        while let Some(op) = self.next_char() {
            if op == '+' || op == '-' {
                match self.next_digit() {
                    Ok(character) => postfix.push(character),
                    Err(error) => return Err(error),
                }
                postfix.push(op);
            } else {
                return Err(InterpreterError::UnexpectedSymbol(op));
            }
        }
        Ok(postfix)
    }

    fn next_digit(&mut self) -> Result<char, InterpreterError> {
        match self.next_char() {
            Some(character) if character.is_digit(10) => Ok(character),
            Some(character) => Err(InterpreterError::UnexpectedSymbol(character)),
            None => Err(InterpreterError::UnextpectedEnd),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_expression() {
        let postfix = Interpreter::new("2+3").interpret().unwrap();
        assert_eq!(postfix, "23+");
    }
    #[test]
    fn mult_expression() {
        let postfix = Interpreter::new("1-2+3-4").interpret().unwrap();
        assert_eq!(postfix, "12-3+4-");
    }
    #[test]
    fn unexpected_symbol() {
        let unexpected_symbol: Result<String, InterpreterError> =
            Interpreter::new("11").interpret();
        assert!(unexpected_symbol.is_err());

        assert!(
            matches!(unexpected_symbol, Err(InterpreterError::UnexpectedSymbol(character)) if character == '1')
        );
    }
    #[test]
    fn unexpected_end() {
        let unexpected_end: Result<String, InterpreterError> = Interpreter::new("1+").interpret();
        assert!(unexpected_end.is_err());

        assert!(matches!(
            unexpected_end,
            Err(InterpreterError::UnextpectedEnd)
        ));
    }
}
