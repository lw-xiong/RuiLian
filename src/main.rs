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
    let source = r#"
        print "=== Function Tests ===";
        
        // Basic function
        fn greet(name) {
            return "Hello, " + name + "!";
        }
        
        print greet("Alice");
        print greet("Bob");
        
        // Function with multiple parameters
        fn add(a, b) {
            return a + b;
        }
        
        print "5 + 3 = " + add(5, 3);
        print "10 + 20 = " + add(10, 20);
        
        // Recursive function (factorial)
        fn factorial(n) {
            if (n <= 1) {
                return 1;
            }
            return n * factorial(n - 1);
        }
        
        print "5! = " + factorial(5);
        print "6! = " + factorial(6);
        
        // Nested function calls
        fn square(x) {
            return x * x;
        }
        
        fn sum_of_squares(a, b) {
            return square(a) + square(b);
        }
        
        print "Sum of squares 3, 4 = " + sum_of_squares(3, 4);
        
        // Function without return (returns 0)
        fn say_hello() {
            print "Hello from function!";
        }
        
        let result = say_hello();
        print "Function without return: " + result;
        
        // Local variables in functions
        fn calculate_discount(price, percent) {
            let discount = price * percent / 100;
            return price - discount;
        }
        
        print "Price 100 with 20% discount: " + calculate_discount(100, 20);
        
        // Function accessing outer scope
        let multiplier = 2;
        fn multiply(x) {
            return x * multiplier;  // Can access outer variable
        }
        
        print "3 * 2 = " + multiply(3);
        
        // Function with early return
        fn absolute(x) {
            if (x >= 0) {
                return x;
            }
            return -x;
        }
        
        print "Absolute of -10: " + absolute(-10);
        print "Absolute of 7: " + absolute(7);
        
        print "=== All function tests passed! ===";
    "#;

    println!("=== Testing Functions ===\n");

    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    let mut parser = Parser::new(tokens);
    let program = parser.parse();

    let mut interpreter = Interpreter::new();
    interpreter.interpret(&program);

    println!("\n=== Function Test Complete ===");
}
