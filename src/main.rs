<<<<<<< HEAD
mod ast;
mod environment;
mod interpreter;
mod parser;
mod scanner;
mod tokens;

use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;

fn main() {
    // Minimal test to debug the loop
    let source = r#"
        let i = 0;
        print i;
        i = i + 1;  // Test assignment first
        print i;
        
        // Then test the loop
        i = 0;
        while (i < 3) {
            print i;
            i = i + 1;
        }
    "#;

    println!("=== Debug Test ===\n");

    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    // Print tokens to verify scanning
    // for token in &tokens {
    //     println!("{:?}", token);
    // }

    let mut parser = Parser::new(tokens);
    let program = parser.parse();

    // Print AST to verify parsing
    // println!("AST: {:#?}", program);

    let mut interpreter = Interpreter::new();
    println!("Output:");
    interpreter.interpret(&program);
}
=======
mod ast;
mod environment;
mod interpreter;
mod parser;
mod scanner;
mod tokens;

use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;

fn main() {
    // Minimal test to debug the loop
    let source = r#"
        let i = 0;
        print i;
        i = i + 1;  // Test assignment first
        print i;
        
        // Then test the loop
        i = 0;
        while (i < 3) {
            print i;
            i = i + 1;
        }
    "#;

    println!("=== Debug Test ===\n");

    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    // Print tokens to verify scanning
    // for token in &tokens {
    //     println!("{:?}", token);
    // }

    let mut parser = Parser::new(tokens);
    let program = parser.parse();

    // Print AST to verify parsing
    // println!("AST: {:#?}", program);

    let mut interpreter = Interpreter::new();
    println!("Output:");
    interpreter.interpret(&program);
}
>>>>>>> f2fa646c3511ab8df1b1775b0c72186b2f2536cf
