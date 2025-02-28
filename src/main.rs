#[derive(Debug)]
enum Op {
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
enum Token {
    Value(f64),
    Operator(Op),
    OpenParen,
    CloseParen,
}

fn tokenize(expr: &str) -> Result<Vec<Token>, String> {
    let mut num_buffer:String = String::new();
    let a = expr
        .chars()
        .filter(|x| { x.to_string() != " " })
        ;

    println!("{:?}", a);
    let mut tokens = a.fold(vec![], |mut acc, token| {
            match token {
                '+' => {
                    check_reset_num_buffer(&mut num_buffer, &mut acc);
                    acc.push(Token::Operator(Op::Add));
                },
                '-' => {
                    check_reset_num_buffer(&mut num_buffer, &mut acc);
                    acc.push(Token::Operator(Op::Sub));
                },
                '*' => {
                    check_reset_num_buffer(&mut num_buffer, &mut acc);
                    acc.push(Token::Operator(Op::Mul));
                },
                '/' => {
                    check_reset_num_buffer(&mut num_buffer, &mut acc);
                    acc.push(Token::Operator(Op::Div));
                },
                '^' => {
                    check_reset_num_buffer(&mut num_buffer, &mut acc);
                    acc.push(Token::Operator(Op::Pow));
                },
                '(' => {
                    check_reset_num_buffer(&mut num_buffer, &mut acc);
                    acc.push(Token::OpenParen);
                },
                ')' => {
                    check_reset_num_buffer(&mut num_buffer, &mut acc);
                    acc.push(Token::CloseParen);
                },
                _ => {
                    num_buffer.push(token);
                }
            };
            acc
        });

    if num_buffer != "" {
        check_reset_num_buffer(&mut num_buffer, &mut tokens);
    }
    Ok(tokens)
}

fn check_reset_num_buffer(num_buffer: &mut String, acc: &mut Vec<Token>) {
    if num_buffer != "" {
        acc.push(Token::Value(num_buffer.parse::<f64>().unwrap()));
        *num_buffer = "".to_string();
    }
}

fn shunting_yard(tokens: Vec<Token>) -> Vec<Token> {
    let (mut output, stack) = tokens.into_iter().fold(
        (Vec::new(), Vec::new()),
        |(mut output, mut stack), token| {
            match token {
                Token::Value(_) => {
                    output.push(token);
                }
                Token::Operator(op) => {
                    // Desempilha operadores de maior ou igual precedência
                    while let Some(Token::Operator(stack_op)) = stack.last() {
                        if precedence(stack_op) >= precedence(&op) {
                            output.push(stack.pop().unwrap());
                        } else {
                            break;
                        }
                    }
                    stack.push(Token::Operator(op)); // Empilha o operador atual
                }

                Token::OpenParen => {
                    stack.push(token);
                }
                Token::CloseParen => {
                    while let Some(token) = stack.pop() {
                        if let Token::OpenParen = token {
                            break;
                        }
                        output.push(token);
                    }
                }
            }
            (output, stack)
        },
    );

    output.extend(stack.into_iter().rev());

    output
}

fn evaluate(rpn: Vec<Token>) -> Result<f64, String> {
    let mut result = rpn.into_iter().fold(Ok(Vec::new()), |output, token| {
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
                output.push(result);
            }

            _ => return Err("Invalid Token in RPN".to_string()),
        }

        Ok(output)
    })?;

    if result.len() == 1 {
        Ok(result.pop().unwrap())
    } else {
        Err("Invalid Expression".to_string())
    }
}

fn main() {
    let expr = "1+2 +(4-5)- (2/4)";
    let tokens = tokenize(expr).unwrap();
    println!("{:?}", tokens);
    let rpn = shunting_yard(tokens);
    let result = evaluate(rpn);
    println!("{:?}", result);
}
