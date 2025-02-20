#[derive(Debug)]
enum Op{
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(Debug)]
enum Token{
    Value(f64),
    Operator(Op),
    OpenParen,
    CloseParen
}

fn tokenize(expr: &str) -> Vec<Token> {
    expr
        .split_whitespace()
        .map(|x| match x {
            "+" => Token::Operator(Op::Add),
            "-" => Token::Operator(Op::Sub),
            "*" => Token::Operator(Op::Mul),
            "/" => Token::Operator(Op::Div),
            "^" => Token::Operator(Op::Pow),
            "(" => Token::OpenParen,
            ")" => Token::CloseParen,
            _ => Token::Value(x
                .parse::<f64>()
                .expect("Invalid token")
            ),
        })
        .collect()
}
fn main() {
    let expr = "3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3";
    let tokens = tokenize(expr);
    println!("{:?}", tokens);
}
