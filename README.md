# Calculadore
Calculadore is a calculator written in Rust, developed for learning purposes.
It uses Reverse Polish Notation to perform calculations respecting the order of operations and priorities.

## Functionalities

- **Operations Already Supported**: Addition **(+)**, Subtraction **(-)**, Multiplication **(*)**, Division **(/)** and Exponentiation **(^)**.
- **Parentheses**: Support for the use of parentheses to define the order of operations.

## How to Use

1. **Clone the Repository**: 
    ```bash
    git clone https://github.com/CoininDev/calculadore.git
    cd calculadore
    ```

2. **Compile the Project**: 
    ```bash
    cargo build
    ```
    NOTE: You need to install [Rust](https://www.rust-lang.org/) to compile the project.

4. **Run the Calculator**: 
    ```bash
    cargo run
    ```

5. **Enter a Mathematical Expression**: 
    - Examples: `2+3*4`, `(1+2)^3`
    - To exit, type `exit`.

## Code Structure

- **Tokenization**: The `tokenize` function converts a string of mathematical expression into a list of tokens.
- **Shunting Yard**: The `shunting_yard` function converts the list of tokens to reverse polish notation (RPN).
- **Evaluation**: The `evaluate` function calculates the value of the expression in RPN.

## Contribution

Contributions are welcome! Feel free to open issues and pull requests.
