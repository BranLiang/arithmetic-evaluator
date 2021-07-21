use super::tokenizer::{Tokenizer, Token};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum OperPrec {
  Default,
  AddSub,
  MulDiv,
  Power,
  Negative
}

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
  Number(f64),
  Add(Box<Node>, Box<Node>),
  Subtract(Box<Node>, Box<Node>),
  Multiply(Box<Node>, Box<Node>),
  Divide(Box<Node>, Box<Node>),
  Power(Box<Node>, Box<Node>),
  Negative(Box<Node>)
}

pub struct Parser<'a> {
  tokenizer: Tokenizer<'a>,
  current_token: Token
}

impl<'a> Parser<'a> {
  pub fn new(expr: &'a str) -> Self {
    let mut tokenizer = Tokenizer::new(expr);
    let first_token = tokenizer.next();
    Parser {
      tokenizer: tokenizer,
      current_token: first_token
    }
  }

  pub fn parse(&mut self) -> Node {
    self.generate_ast(OperPrec::Default)
  }

  fn generate_ast(&mut self, oper_prec: OperPrec) -> Node {
    let mut left_node = self.parse_token();
    while oper_prec < self.current_token.get_oper_prec() {
      if self.current_token == Token::EOF {
        break;
      };
      left_node = self.convert_token_to_node(left_node.clone());
    }
    left_node
  }

  fn convert_token_to_node(&mut self, left_node: Node) -> Node {
    match self.current_token {
      Token::Add => {
        self.get_next_token();
        let right_node = self.generate_ast(OperPrec::AddSub);
        Node::Add(
          Box::new(left_node),
          Box::new(right_node)
        )
      }
      Token::Subtract => {
        self.get_next_token();
        let right_node = self.generate_ast(OperPrec::AddSub);
        Node::Subtract(
          Box::new(left_node),
          Box::new(right_node)
        )
      }
      Token::Divide => {
        self.get_next_token();
        let right_node = self.generate_ast(OperPrec::MulDiv);
        Node::Divide(
          Box::new(left_node),
          Box::new(right_node)
        )
      }
      Token::Multiply => {
        self.get_next_token();
        let right_node = self.generate_ast(OperPrec::MulDiv);
        Node::Multiply(
          Box::new(left_node),
          Box::new(right_node)
        )
      }
      Token::Caret => {
        self.get_next_token();
        let right_node = self.generate_ast(OperPrec::Power);
        Node::Power(
          Box::new(left_node),
          Box::new(right_node)
        )
      }
      _ => panic!("Unsupported token.")
    }
  }

  fn parse_token(&mut self) -> Node {
    match self.current_token {
      Token::Number(i) => {
        self.get_next_token();
        Node::Number(i)
      },
      Token::Subtract => {
        self.get_next_token();
        let node = self.generate_ast(OperPrec::Negative);
        Node::Negative(Box::new(node))
      }
      Token::LeftParen => {
        self.get_next_token();
        let mut node = self.generate_ast(OperPrec::Default);
        if self.current_token != Token::RightParen {
          panic!("Missing right parenthesis");
        }
        self.get_next_token();
        if self.current_token == Token::LeftParen {
          let right_node = self.generate_ast(OperPrec::MulDiv);
          node = Node::Multiply(
            Box::new(node),
            Box::new(right_node)
          )
        }
        node
      }
      _ => panic!("Unsupported token")
    }
  }

  fn get_next_token(&mut self) {
    let token = self.tokenizer.next();
    self.current_token = token;
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn parse_single_number() {
    let expr = "1";
    let mut parser = Parser::new(expr);
    assert_eq!(parser.parse(), Node::Number(1_f64));
  }

  #[test]
  fn parse_simple_add_expression() {
    let expr = "1+2";
    let mut parser = Parser::new(expr);
    assert_eq!(parser.parse(), Node::Add(
      Box::new(Node::Number(1_f64)), Box::new(Node::Number(2_f64))
    ))
  }

  #[test]
  fn parse_simple_power_operation() {
    let expr = "2^3";
    let mut parser = Parser::new(expr);
    assert_eq!(parser.parse(), Node::Power(
      Box::new(Node::Number(2_f64)), Box::new(Node::Number(3_f64))
    ))
  }

  #[test]
  fn parse_expression_with_parenthesis() {
    let expr = "2*(1+3)";
    let mut parser = Parser::new(expr);
    assert_eq!(parser.parse(), Node::Multiply(
      Box::new(Node::Number(2_f64)),
      Box::new(
        Node::Add(
          Box::new(Node::Number(1_f64)),
          Box::new(Node::Number(3_f64))
        )
      )
    ))
  }

  #[test]
  fn parse_expression_with_negative_value() {
    let expr = "-2^3";
    let mut parser = Parser::new(expr);
    assert_eq!(parser.parse(), Node::Power(
      Box::new(Node::Negative(
        Box::new(Node::Number(2_f64))
      )), 
      Box::new(Node::Number(3_f64))
    ))
  }

  #[test]
  #[should_panic(expected = "Missing right parenthesis")]
  fn parenthesis_must_in_pair() {
    let expr = "2*(1+3";
    let mut parser = Parser::new(expr);
    parser.parse();
  }

  #[test]
  fn parse_expression_with_parenthesis_with_after_actions() {
    let expr = "(1+3)*2";
    let mut parser = Parser::new(expr);
    assert_eq!(parser.parse(), Node::Multiply(
      Box::new(
        Node::Add(
          Box::new(Node::Number(1_f64)),
          Box::new(Node::Number(3_f64))
        )
      ),
      Box::new(Node::Number(2_f64))
    ))
  }

  #[test]
  fn parse_expression_with_two_parenthesis() {
    let expr = "(1+3)(2+4)";
    let mut parser = Parser::new(expr);
    assert_eq!(parser.parse(), Node::Multiply(
      Box::new(
        Node::Add(
          Box::new(Node::Number(1_f64)),
          Box::new(Node::Number(3_f64))
        )
      ),
      Box::new(
        Node::Add(
          Box::new(Node::Number(2_f64)),
          Box::new(Node::Number(4_f64))
        )
      ),
    ))
  }

  #[test]
  fn parse_complex_expr() {
    let expr = "-1 + 2.4 * 3 - 4^2 / (1.5 + 1)(1+2)";
    let mut parser = Parser::new(expr);
    assert_eq!(parser.parse(),
      Node::Subtract(
        Box::new(
          Node::Add(
            Box::new(
              Node::Negative(Box::new(Node::Number(1_f64)))
            ),
            Box::new(
              Node::Multiply(
                Box::new(Node::Number(2.4_f64)),
                Box::new(Node::Number(3_f64))
              )
            )
          )
        ),
        Box::new(
          Node::Divide(
            Box::new(
              Node::Power(
                Box::new(Node::Number(4_f64)),
                Box::new(Node::Number(2_f64))
              )
            ),
            Box::new(
              Node::Multiply(
                Box::new(
                  Node::Add(
                    Box::new(Node::Number(1.5_f64)),
                    Box::new(Node::Number(1_f64))
                  )
                ),
                Box::new(
                  Node::Add(
                    Box::new(Node::Number(1_f64)),
                    Box::new(Node::Number(2_f64))
                  )
                ),
              )
            )
          )
        )
      )
    )
  }
}