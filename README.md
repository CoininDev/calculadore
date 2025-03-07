# Calculadore
A simple Calculator CLI made with Rust.

## How it works
The calculator reads an expressions, and then turns it into intelligible tokens, then it converts the expression into Reverse Polish Notation for easier evaluation, an finally evaluates and prints the result, repeating the process until user type "exit".

## How to use
1. Clone the repository
   `git clone https://github.com/CoininDev/calculadore && cd calculadore`
2. Run the code (OBS: You need to install [rust](https://www.rust-lang.org/) first)
   `cargo run`
3. Type a mathematic expression, "func=" to declare a function, "f(value)" to use it and "exit" to exit.


## What's already working
✅ Basic arithmetic (`+`, `-`, `*`, `/`, `^`)  
✅ Parentheses for precedence
✅ Function definitions (`func=x^2`)  
✅ Evaluate functions (`f(5)`)

## What's not done yet
🔧 Support for multi-variable functions (e.g., f(x,y))
🔧 Embed functions within larger expressions
🔧 CLI enhancements (arrow keys, history, etc.)

