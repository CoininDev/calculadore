#[derive(Debug)]
enum Op{
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

enum Token{
    Value(f64),
    Operator(Op),
    OpenParen,
    CloseParen
}

fn main() {
    println!("Hello, world!");
}
