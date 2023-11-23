# SamoraLang Rust Lexical Analyzer

The SamoraLang Rust Lexical Analyzer is a lexer implemented in Rust for the SamoraLang programming language.
This lexer is designed to tokenize SamoraLang source code, 
providing a foundation for further stages in the language processing pipeline.

## Features

- Tokenizes SamoraLang source code into a stream of tokens.
- Supports a variety of token types, including identifiers, keywords, operators, and literals.
- Handles whitespace and comments gracefully.
- Written in Rust for efficiency and reliability.

## Getting Started

### Prerequisites

To use the SamoraLang Rust Lexical Analyzer, you need to have Rust and Cargo installed on your system. If you haven't installed Rust, you can do so by following the instructions on the [official Rust website](https://www.rust-lang.org/).

### Installation

Clone the repository to your local machine:

```bash
git clone https://github.com/GraHms/SLang-Rust-Lexical-Analyzer.git
cd SLang-Rust-Lexical-Analyzer.
```

Build the project using Cargo:

```bash
cargo build --release
```

### Usage

Once the project is built, you can use the lexer in your Rust projects. Import the lexer module and create an instance of the `Lexer` struct:

```rust
use samoralang_lexer_rust::lexer::Lexer;

fn main() {
    let input = "let x = 5;";
    let mut lexer = Lexer::new(input.to_string());

    loop {
        let token = lexer.next_token();
        // Process or display the token as needed
        println!("{:?}", token);

        if token.r#type == TokenType::EOF {
            break;
        }
    }
}
```

## Contributing

If you find issues or want to contribute to the development of the SamoraLang Rust Lexical Analyzer, please open an issue or submit a pull request on the [GitHub repository](https://github.com/GraHms/SLang-Rust-Lexical-Analyzer.git).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.


---

Feel free to customize this template based on your specific project details. Include any additional sections that might be relevant for your users, such as a roadmap, known issues, or examples of SamoraLang code.