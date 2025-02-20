#[derive(Debug)]
enum Op{
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

fn precedence(op: &Op) -> u8 {
    match op {
        Op::Add | Op::Sub => 1,
        Op::Mul | Op::Div => 2,
        Op::Pow => 3,
    }
}

#[derive(Debug)]
enum Token{
    Value(f64),
    Operator(Op),
    OpenParen,
    CloseParen
}

fn tokenize(expr: &str) -> Result<Vec<Token>, String> {
    expr
        .split_whitespace()
        .map(|x| match x {
            "+" => Ok(Token::Operator(Op::Add)),
            "-" => Ok(Token::Operator(Op::Sub)),
            "*" => Ok(Token::Operator(Op::Mul)),
            "/" => Ok(Token::Operator(Op::Div)),
            "^" => Ok(Token::Operator(Op::Pow)),
            "(" => Ok(Token::OpenParen),
            ")" => Ok(Token::CloseParen),
            num => num
                            .parse::<f64>()
                            .map(Token::Value)
                            .map_err(|e| format!("Invalid Number: {}", e)),
        })
        .collect()
}

fn main() {
    let expr = "3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3";
    let tokens = tokenize(expr);
    if let Ok(tokens) = &tokens {
        println!("{:?}", tokens);
    }
}
