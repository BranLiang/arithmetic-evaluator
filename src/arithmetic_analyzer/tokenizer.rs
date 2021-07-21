use std::iter::Peekable;
use std::str::Chars;
use super::parser::OperPrec;

#[derive(PartialEq, Debug)]
pub enum Token {
  Number(f64),
  Add,
  Subtract,
  Multiply,
  Divide,
  Caret,
  LeftParen,
  RightParen,
  EOF
}

impl Token {
  pub fn get_oper_prec(&self) -> OperPrec {
    use self::Token::*;

    match self {
      Add | Subtract => OperPrec::AddSub,
      Divide | Multiply => OperPrec::MulDiv,
      Caret => OperPrec::Power,
      _ => OperPrec::Default
    }
  }
}

pub struct Tokenizer<'a> {
  expr: Peekable<Chars<'a>>
}

impl<'a> Tokenizer<'a> {
  pub fn new(expr: &'a str) -> Self {
    Tokenizer {
      expr: expr.chars().peekable()
    }
  }

  pub fn next(&mut self) -> Token {
    match self.expr.next() {
      Some('+') => Token::Add,
      Some('-') => Token::Subtract,
      Some('^') => Token::Caret,
      Some('*') => Token::Multiply,
      Some('/') => Token::Divide,
      Some('(') => Token::LeftParen,
      Some(')') => Token::RightParen,
      Some(n @ '0'..='9') => {
        let mut number = n.to_string();
        while let Some(&next_char) = self.expr.peek() {
          if next_char == '.' || (next_char >= '0' && next_char <= '9') {
            self.expr.next();
            number.push(next_char)
          } else if next_char == '_' || next_char == ' ' {
            self.expr.next();
            continue
          } else {
            break
          }
        }
        Token::Number(number.parse::<f64>().unwrap())
      }
      Some(' ') => self.next(),
      Some(_) => panic!("Invalid character"),
      None => Token::EOF
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn next_returns_expected_tokens() {
    let mut tokenizer = Tokenizer::new("1+12.15*5/1^2 - (6. 1-1)");

    assert_eq!(tokenizer.next(), Token::Number(1_f64));
    assert_eq!(tokenizer.next(), Token::Add);
    assert_eq!(tokenizer.next(), Token::Number(12.15_f64));
    assert_eq!(tokenizer.next(), Token::Multiply);
    assert_eq!(tokenizer.next(), Token::Number(5_f64));
    assert_eq!(tokenizer.next(), Token::Divide);
    assert_eq!(tokenizer.next(), Token::Number(1_f64));
    assert_eq!(tokenizer.next(), Token::Caret);
    assert_eq!(tokenizer.next(), Token::Number(2_f64));
    assert_eq!(tokenizer.next(), Token::Subtract);
    assert_eq!(tokenizer.next(), Token::LeftParen);
    assert_eq!(tokenizer.next(), Token::Number(6.1_f64));
    assert_eq!(tokenizer.next(), Token::Subtract);
    assert_eq!(tokenizer.next(), Token::Number(1_f64));
    assert_eq!(tokenizer.next(), Token::RightParen);
    assert_eq!(tokenizer.next(), Token::EOF);
  }
}