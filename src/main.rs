use std::io::{self, Write};

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
    let mut tokens = expr
        .chars()
        .filter(|x| { x.to_string() != " " })
        .fold(vec![], |mut acc, token| {
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
    println!("Type the mathematic expression (e.g.: 2+2) or \"exit\" to leave.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap(); 

        let mut expr = String::new();
        io::stdin()
            .read_line(&mut expr)
            .expect("Failed to read line.");

        let expr = expr.trim(); 

        if expr == "exit" {
            println!("Exiting...");
            break; 
        }

        let tokens = match tokenize(&expr) {
            Ok(tokens) => tokens,
            Err(e) => {
                println!("Error: {}", e);
                continue; 
            }
        };

        let rpn = shunting_yard(tokens);
        println!("Internal Tokens: {:?}", rpn);
        let result = match evaluate(rpn) {
            Ok(result) => result,
            Err(e) => {
                println!("Error: {}", e);
                continue; 
            }
        };
        
        println!("Result: {:?}", result);
    }
}
