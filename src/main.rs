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
        print "=== Array Tests ===";
        
        // Basic array creation
        let numbers = [1, 2, 3, 4, 5];
        print numbers;
        
        // Array of strings
        let names = ["Alice", "Bob", "Charlie"];
        print names;
        
        // Mixed array (using integer instead of float)
        let mixed = [1, "hello", true, 314];
        print mixed;
        
        // Array indexing
        print "First number: " + numbers[0];
        print "Second name: " + names[1];
        
        // Array length
        print "Numbers length: " + len(numbers);
        print "Names length: " + len(names);
        
        // Iterating over array with while loop
        let i = 0;
        while (i < len(numbers)) {
            print "numbers[" + i + "] = " + numbers[i];
            i = i + 1;
        }
        
        // Array in function
        fn sum_array(arr) {
            let total = 0;
            let i = 0;
            while (i < len(arr)) {
                total = total + arr[i];
                i = i + 1;
            }
            return total;
        }
        
        print "Sum of numbers: " + sum_array(numbers);
        
        // Empty array
        let empty = [];
        print "Empty array: " + empty;
        print "Empty array length: " + len(empty);
        
        // Array truthiness
        if (empty) {
            print "Empty array is truthy";
        } else {
            print "Empty array is falsy";
        }
        
        if (numbers) {
            print "Non-empty array is truthy";
        }
        
        print "=== Array tests complete ===";


        print "=== For Loop Tests ===";
        
        // Basic array iteration
        let numbers = [1, 2, 3, 4, 5];
        for (num in numbers) {
            print "Number: " + num;
        }
        
        // String iteration
        let word = "hello";
        for (char in word) {
            print "Character: " + char;
        }
        
        // Nested loops
        let matrix = [[1, 2], [3, 4], [5, 6]];
        for (row in matrix) {
            print "Row: " + row;
            for (element in row) {
                print "  Element: " + element;
            }
        }
        
        // Sum with for loop
        let numbers = [10, 20, 30, 40, 50];
        let sum = 0;
        for (n in numbers) {
            sum = sum + n;
        }
        print "Sum: " + sum;
        
        // Empty array
        let empty = [];
        for (item in empty) {
            print "This won't print";
        }
        print "Empty loop completed";
        
        print "=== For Loop Tests Complete ===";


         print "=== Hash Map Tests ===";
    
    // Basic map creation
    let person = { "name": "Alice", "age": 30 };
    print person;
    print "Name: " + person["name"];
    print "Age: " + person["age"];
    
    // Map mutation
    person["age"] = 31;
    print "Updated age: " + person["age"];
    
    // Add new key
    person["city"] = "New York";
    print "City: " + person["city"];
    print "Full person: " + person;
    
    // Nested structures
    let company = {
        "name": "Tech Corp",
        "employees": [
            { "name": "Alice", "role": "Engineer" },
            { "name": "Bob", "role": "Designer" }
        ]
    };
    print "Company: " + company;
    print "First employee: " + company["employees"][0];
    
    // Iterate over map keys (we'll need to add keys() method later)
    print "Keys in person:";
    // TODO: Add map.keys() method
    
    // Default value for missing keys
    print "Missing key: " + person["nonexistent"];
    
    // Empty map
    let empty = {};
    print "Empty map: " + empty;
    
    print "=== Hash Map Tests Complete ===";

    
    print "=== Dot Notation Tests ===";
    
    // Basic dot notation
    let person = { "name": "Alice", "age": 30 };
    print "Name via bracket: " + person["name"];
    print "Name via dot: " + person.name;
    print "Age via dot: " + person.age;
    
    // Dot assignment
    person.age = 31;
    print "Updated age: " + person.age;
    
    // Add new field with dot notation
    person.city = "New York";
    print "City: " + person.city;
    
    // Nested structures with dot notation
    let company = {
        "name": "Tech Corp",
        "address": {
            "street": "123 Main St",
            "city": "Boston"
        },
        "employees": [
            { "name": "Alice", "role": "Engineer" },
            { "name": "Bob", "role": "Designer" }
        ]
    };
    
    print "Company: " + company.name;
    print "Address: " + company.address.street + ", " + company.address.city;
    print "First employee: " + company.employees[0].name;
    
    // Mix bracket and dot notation
    print "First employee role: " + company.employees[0]["role"];
    
    // Chained dot notation
    company.employees[0].role = "Senior Engineer";
    print "Updated role: " + company.employees[0].role;
    
    // Default value for missing fields
    print "Missing field: " + person.nonexistent;  // Should print 0
    
    // Functions as methods (later)
    // person.greet = fn() { return "Hello, " + this.name; };
    // print person.greet();
    
    print "=== Dot Notation Tests Complete ===";
    "#;

    println!("=== Running Tests ===\n");

    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    let mut parser = Parser::new(tokens);
    let program = parser.parse();

    let mut interpreter = Interpreter::new();
    interpreter.interpret(&program);

    println!("\n=== Tests Complete ===");
}
