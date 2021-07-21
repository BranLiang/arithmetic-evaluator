use super::parser::{Node, Parser};

pub fn eval(expr: &str) -> f64 {
  let mut parser = Parser::new(expr);
  let node = parser.parse();
  calculate(node)
}

fn calculate(node: Node) -> f64 {
  match node {
    Node::Number(i) => i,
    Node::Add(x, y) => calculate(*x) + calculate(*y),
    Node::Divide(x, y) => calculate(*x) / calculate(*y),
    Node::Negative(i) => -calculate(*i),
    Node::Multiply(x, y) => calculate(*x) * calculate(*y),
    Node::Subtract(x, y) => calculate(*x) - calculate(*y),
    Node::Power(x, y) => calculate(*x).powf(calculate(*y)),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn evaluate_expr() {
    let result = eval("1 + 2*3.5 - 4 / 2^2 + (1 + (2 * 2)) * 5");
    assert_eq!(result, 32_f64);
  }
}
