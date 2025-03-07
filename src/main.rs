use std::io::{self, Write};

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
enum Token {
    Value(f64),
    Variable(String),
    Operator(Op),
    OpenParen,
    CloseParen,
}

fn tokenize(expr: &str) -> Result<Vec<Token>, String> {
    let mut num_buffer = String::new();
    let mut tokens = Vec::new();

    for c in expr.chars().filter(|c| !c.is_whitespace()) {
        match c {
            '+' | '-' | '*' | '/' | '^' | '(' | ')' => {
                check_reset_num_buffer(&mut num_buffer, &mut tokens)?;
                match c {
                    '+' => tokens.push(Token::Operator(Op::Add)),
                    '-' => tokens.push(Token::Operator(Op::Sub)),
                    '*' => tokens.push(Token::Operator(Op::Mul)),
                    '/' => tokens.push(Token::Operator(Op::Div)),
                    '^' => tokens.push(Token::Operator(Op::Pow)),
                    '(' => tokens.push(Token::OpenParen),
                    ')' => tokens.push(Token::CloseParen),
                    _ => unreachable!(),
                }
            }
            _ => {
                num_buffer.push(c);
            }
        }
    }

    check_reset_num_buffer(&mut num_buffer, &mut tokens)?;

    Ok(tokens)
}

fn check_reset_num_buffer(num_buffer: &mut String, acc: &mut Vec<Token>) -> Result<(), String> {
    if !num_buffer.is_empty() {
        if let Ok(num) = num_buffer.parse::<f64>() {
            acc.push(Token::Value(num));
        } else {
            if num_buffer.chars().all(|c| c.is_alphabetic()) {
                acc.push(Token::Variable(num_buffer.clone()));
            } else {
                return Err(format!("Invalid token: '{}'", num_buffer));
            }
        }
        num_buffer.clear();
    }
    Ok(())
}

fn shunting_yard(tokens: Vec<Token>) -> Vec<Token> {
    let (mut output, mut stack) = tokens.into_iter().fold(
        (Vec::new(), Vec::new()),
        |(mut output, mut stack), token| {
            match token {
                Token::Value(_) | Token::Variable(_) => output.push(token),
                Token::Operator(op) => {
                    while let Some(Token::Operator(stack_op)) = stack.last() {
                        if precedence(stack_op) >= precedence(&op) {
                            output.push(stack.pop().unwrap());
                        } else {
                            break;
                        }
                    }
                    stack.push(Token::Operator(op));
                }
                Token::OpenParen => stack.push(token),
                Token::CloseParen => {
                    while let Some(top) = stack.pop() {
                        if let Token::OpenParen = top {
                            break;
                        }
                        output.push(top);
                    }
                }
            }
            (output, stack)
        },
    );

    while let Some(op) = stack.pop() {
        output.push(op);
    }

    output
}

fn evaluate(rpn: Vec<Token>) -> Result<f64, String> {
    let mut stack = Vec::new();

    for token in rpn {
        match token {
            Token::Value(val) => stack.push(val),
            Token::Operator(op) => {
                let r = stack.pop().ok_or("Not enough operands".to_string())?;
                let l = stack.pop().ok_or("Not enough operands".to_string())?;
                let result = match op {
                    Op::Add => l + r,
                    Op::Sub => l - r,
                    Op::Mul => l * r,
                    Op::Div => l / r,
                    Op::Pow => l.powf(r),
                };
                stack.push(result);
            }
            _ => return Err("Invalid token in RPN".to_string()),
        }
    }

    if stack.len() == 1 {
        Ok(stack.pop().unwrap())
    } else {
        Err("Invalid expression".to_string())
    }
}

fn apply_variable(tokens: Vec<Token>, var: f64) -> Vec<Token> {
    tokens
        .into_iter()
        .map(|token| match token {
            Token::Variable(_) => Token::Value(var),
            t => t,
        })
        .collect()
}

fn main() {
    println!("Type a mathematical expression (e.g., 2+2), define a function with 'func=...', apply it with 'f(value)', or type 'exit' to quit.");
    let mut func: Option<Vec<Token>> = None;

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut expr = String::new();
        io::stdin()
            .read_line(&mut expr)
            .expect("Failed to read line.");
        let expr = expr.trim();

        if expr.is_empty() {
            continue;
        }

        if expr == "exit" {
            println!("Exiting...");
            break;
        }

        if let Some(stripped) = expr.strip_prefix("func=") {
            match tokenize(stripped) {
                Ok(tokens) => {
                    let rpn = shunting_yard(tokens);
                    func = Some(rpn);
                    println!("Function defined.");
                }
                Err(e) => println!("Error: {}", e),
            }
            continue;
        }

        if let Some(ref func_tokens) = func {
            if expr.starts_with('f') {
                let arg_expr = if let Some(stripped) = expr.strip_prefix("f=") {
                    stripped
                } else if expr.starts_with("f(") && expr.ends_with(')') {
                    &expr[2..expr.len() - 1]
                } else {
                    println!("Invalid function application syntax. Use 'f=value' or 'f(value)'.");
                    continue;
                };

                let tokens = match tokenize(arg_expr) {
                    Ok(t) => t,
                    Err(e) => {
                        println!("Error tokenizing argument: {}", e);
                        continue;
                    }
                };
                let rpn = shunting_yard(tokens);
                let x = match evaluate(rpn) {
                    Ok(val) => val,
                    Err(e) => {
                        println!("Error evaluating argument: {}", e);
                        continue;
                    }
                };

                let substituted = apply_variable(func_tokens.clone(), x);
                match evaluate(substituted) {
                    Ok(result) => println!("Result: {}", result),
                    Err(e) => println!("Error evaluating function: {}", e),
                }
                continue;
            }
        }

        match tokenize(expr) {
            Ok(tokens) => {
                let rpn = shunting_yard(tokens);
                match evaluate(rpn) {
                    Ok(result) => println!("Result: {}", result),
                    Err(e) => println!("Error: {}", e),
                }
            }
            Err(e) => println!("Error tokenizing: {}", e),
        }
    }
}
