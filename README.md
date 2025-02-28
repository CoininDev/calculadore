# Calculadore
Calculadore é uma calculadora escrita em Rust, desenvolvida a fins de aprendizado. 
Ela usa Notação Polonesa Reversa para realizar as contas respeitando a ordem de operções e prioridades.

## Funcionalidades

- **Operações Já Suportadas**: Adição **(+)**, Subtração **(-)**, Multiplicação **(*)**, Divisão **(/)** e Exponenciação **(^)**.
- **Parênteses**: Suporte ao uso de parênteses para definir a ordem das operações.

## Como Usar

1. **Clone o Repositório**:
    ```bash
    git clone https://github.com/CoininDev/calculadore.git
    cd calculadore
    ```

2. **Compile o Projeto**:
    ```bash
    cargo build
    ```
    OBS: É preciso instalar [Rust](https://www.rust-lang.org/) para compilar o projeto.

4. **Execute a Calculadora**:
    ```bash
    cargo run
    ```

5. **Digite uma Expressão Matemática**:
    - Exemplos: `2+3*4`, `(1+2)^3`
    - Para sair, digite `exit`.

## Estrutura do Código

- **Tokenização**: A função `tokenize` converte uma string de expressão matemática em uma lista de tokens.
- **Shunting Yard**: A função `shunting_yard` converte a lista de tokens para a notação polonesa reversa (RPN).
- **Avaliação**: A função `evaluate` calcula o valor da expressão em RPN.

## Contribuição

Contribuições são bem-vindas! Sinta-se à vontade para abrir issues e pull requests.
