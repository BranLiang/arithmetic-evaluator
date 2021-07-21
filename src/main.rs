mod arithmetic_analyzer;

use std::env;

fn main() {
    let expr = env::args().nth(1).unwrap();
    let result = arithmetic_analyzer::evaluator::eval(expr.as_str());
    println!("Result: {}", result);
}
