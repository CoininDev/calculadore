use std::result;

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

fn shunting_yard(tokens: Vec<Token>) -> Vec<Token> {
    let (mut output, stack) = tokens
        .into_iter()
        .fold((Vec::new(), Vec::new()), |(mut output, mut stack), token| {
            match token {
                Token::Value(_) => {output.push(token);}
                Token::Operator(op) => {
                    // Desempilha operadores de maior ou igual precedência
                    while let Some(Token::Operator(stack_op)) = stack.last() {
                        if precedence(stack_op) >= precedence(&op) {
                            output.push(stack.pop().unwrap());
                        } else { break; }
                    }
                    stack.push(Token::Operator(op)); // Empilha o operador atual
                }

                Token::OpenParen => {stack.push(token);},
                Token::CloseParen => {
                    while let Some(token) = stack.pop() {
                        if let Token::OpenParen = token { break; }
                        output.push(token);
                    }
                }
            }
            (output, stack)
        });

    output.extend(stack.into_iter().rev());

    output
}

fn evaluate(rpn: Vec<Token>) -> Result<f64, String>{
    let mut result = rpn
        .into_iter()
        .fold(Ok(Vec::new()), |output, token| {
            let mut output = output?;

            match token {
                Token::Value(val) => output.push(val),
                Token::Operator(op) => {
                    let r = output.pop().ok_or("Erro!")?;
                    let l = output.pop().ok_or("err")?;
                    let result = match op {
                        Op::Add => l + r,
                        Op::Sub => l - r,
                        Op::Mul => l * r,
                        Op::Div => l / r,
                        Op::Pow => l.powf(r),
                    };
                }

                _ => return Err("Invalid Token in RPN".to_string())
            }

            Ok(output)
        })?
    ;

    if result.len() == 1 {
        Ok(result.pop().unwrap())
    } else {
        Err("Invalid Expression".to_string())
    }
}

fn main() {
    let expr = "3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3";
    let tokens = tokenize(expr);
    if let Ok(tokens) = &tokens {
        println!("Tokens: {:?}", tokens);
    }

    let rpn = shunting_yard(tokens.unwrap());
    println!("RPN: {:?}", rpn);

    let result = evaluate(rpn);
    println!("Resultado: {:?}", result);

}
