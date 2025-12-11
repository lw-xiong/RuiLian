use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::{BinOp, Expr, LogicalOp, Program, Stmt, UnaryOp};
use crate::environment::{Environment, Function, Value};

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, program: &Program) {
        for stmt in &program.statements {
            if let Err(return_value) = self.execute(stmt) {
                // Top-level return should not happen, but if it does, ignore
                println!(
                    "Warning: Top-level return value ignored: {:?}",
                    return_value
                );
            }
        }
    }

    // Returns Ok(()) for normal execution, Err(Value) for return statement
    fn execute(&mut self, stmt: &Stmt) -> Result<(), Value> {
        match stmt {
            Stmt::Expr(expr) => {
                self.evaluate(expr);
                Ok(())
            }
            Stmt::Let { name, initializer } => {
                let value = match initializer {
                    Some(expr) => self.evaluate(expr),
                    None => Value::Number(0),
                };
                self.environment.borrow_mut().define(name.clone(), value);
                Ok(())
            }
            Stmt::Print(expr) => {
                let value = self.evaluate(expr);
                match value {
                    Value::Number(n) => println!("{}", n),
                    Value::String(s) => println!("{}", s),
                    Value::Boolean(b) => println!("{}", b),
                    Value::Function(func) => println!("<function {}>", func.name),
                }
                Ok(())
            }
            Stmt::Block(statements) => {
                // Enter new scope
                let new_env = Environment::new_enclosed(&self.environment);
                let previous_env = self.environment.clone();
                self.environment = new_env;

                // Execute statements in the block
                let mut result = Ok(());
                for stmt in statements {
                    result = self.execute(stmt);
                    if result.is_err() {
                        break; // Return statement encountered
                    }
                }

                // Exit scope
                self.environment = previous_env;
                result
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition_value = self.evaluate(condition);
                if is_truthy(&condition_value) {
                    self.execute(then_branch)
                } else if let Some(else_branch) = else_branch {
                    self.execute(else_branch)
                } else {
                    Ok(())
                }
            }
            Stmt::While { condition, body } => {
                while is_truthy(&self.evaluate(condition)) {
                    let result = self.execute(body);
                    if result.is_err() {
                        return result; // Return statement in loop body
                    }
                }
                Ok(())
            }
            Stmt::Function { name, params, body } => {
                let function = Function {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                    closure: Rc::clone(&self.environment),
                };
                self.environment
                    .borrow_mut()
                    .define(name.clone(), Value::Function(function));
                Ok(())
            }
            Stmt::Return { value } => {
                let return_value = match value {
                    Some(expr) => self.evaluate(expr),
                    None => Value::Number(0),
                };
                Err(return_value) // Signal return with Err
            }
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Number(n) => Value::Number(*n),
            Expr::String(s) => Value::String(s.clone()),
            Expr::Boolean(b) => {
                if *b {
                    Value::Boolean(true)
                } else {
                    Value::Boolean(false)
                }
            }
            Expr::Variable(name) => self
                .environment
                .borrow()
                .get(name)
                .unwrap_or_else(|| panic!("Undefined variable '{}'", name)),
            Expr::Assign(name, expr) => {
                let value = self.evaluate(expr);
                if !self.environment.borrow_mut().assign(name, value.clone()) {
                    panic!("Undefined variable '{}' in assignment", name);
                }
                value
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_val = self.evaluate(left);
                let right_val = self.evaluate(right);

                match operator {
                    BinOp::Add => add_values(&left_val, &right_val),
                    BinOp::Subtract => subtract_values(&left_val, &right_val),
                    BinOp::Multiply => multiply_values(&left_val, &right_val),
                    BinOp::Divide => divide_values(&left_val, &right_val),
                    BinOp::Greater => compare_greater(&left_val, &right_val),
                    BinOp::GreaterEqual => compare_greater_equal(&left_val, &right_val),
                    BinOp::Less => compare_less(&left_val, &right_val),
                    BinOp::LessEqual => compare_less_equal(&left_val, &right_val),
                    BinOp::EqualEqual => compare_equal(&left_val, &right_val),
                    BinOp::BangEqual => compare_not_equal(&left_val, &right_val),
                }
            }
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left_val = self.evaluate(left);

                match operator {
                    LogicalOp::And => {
                        if !is_truthy(&left_val) {
                            return Value::Boolean(false);
                        }
                        self.evaluate(right)
                    }
                    LogicalOp::Or => {
                        if is_truthy(&left_val) {
                            return Value::Boolean(true);
                        }
                        self.evaluate(right)
                    }
                }
            }
            Expr::Unary { operator, right } => {
                let right_val = self.evaluate(right);
                match operator {
                    UnaryOp::Negate => match right_val {
                        Value::Number(n) => Value::Number(-n),
                        _ => panic!("Cannot negate non-number"),
                    },
                    UnaryOp::Not => Value::Boolean(!is_truthy(&right_val)),
                }
            }
            Expr::Call { callee, arguments } => {
                let callee_value = self.evaluate(callee);

                match callee_value {
                    Value::Function(function) => {
                        // Check argument count
                        if arguments.len() != function.params.len() {
                            panic!(
                                "Expected {} arguments but got {}",
                                function.params.len(),
                                arguments.len()
                            );
                        }

                        // Create new environment for the function call
                        let call_env = Environment::new_enclosed(&function.closure);

                        // Evaluate arguments in current environment
                        let arg_values: Vec<Value> =
                            arguments.iter().map(|arg| self.evaluate(arg)).collect();

                        // Bind parameters in the new environment
                        for (param, arg_value) in function.params.iter().zip(arg_values) {
                            call_env.borrow_mut().define(param.clone(), arg_value);
                        }

                        // Save current environment and switch to call environment
                        let previous_env = self.environment.clone();
                        self.environment = call_env;

                        // Execute function body
                        let mut return_value = Value::Number(0);
                        let mut return_occurred = false;

                        for stmt in &function.body {
                            match self.execute(stmt) {
                                Ok(()) => continue,
                                Err(value) => {
                                    return_value = value;
                                    return_occurred = true;
                                    break;
                                }
                            }
                        }

                        // Restore environment
                        self.environment = previous_env;

                        // If no return statement executed, return 0
                        if return_occurred {
                            return_value
                        } else {
                            Value::Number(0)
                        }
                    }
                    _ => {
                        // Built-in functions
                        if let Expr::Variable(name) = callee.as_ref() {
                            if name == "print" {
                                for arg in arguments {
                                    let value = self.evaluate(arg);
                                    match value {
                                        Value::Number(n) => print!("{} ", n),
                                        Value::String(s) => print!("{} ", s),
                                        Value::Boolean(b) => print!("{} ", b),
                                        Value::Function(func) => {
                                            print!("<function {}> ", func.name)
                                        }
                                    }
                                }
                                println!();
                                return Value::Number(0);
                            }
                        }
                        panic!("Can only call functions");
                    }
                }
            }
        }
    }
}

fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Number(n) => *n != 0,
        Value::String(s) => !s.is_empty(),
        Value::Boolean(b) => *b,
        Value::Function(_) => true, // Functions are truthy
    }
}

fn add_values(left: &Value, right: &Value) -> Value {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
        (Value::String(a), Value::String(b)) => Value::String(a.clone() + b),
        (Value::String(a), Value::Number(b)) => Value::String(a.clone() + &b.to_string()),
        (Value::Number(a), Value::String(b)) => Value::String(a.to_string() + b),
        _ => panic!("Cannot add {:?} and {:?}", left, right),
    }
}
fn subtract_values(left: &Value, right: &Value) -> Value {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
        _ => panic!("Cannot subtract {:?} from {:?}", right, left),
    }
}

fn multiply_values(left: &Value, right: &Value) -> Value {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
        _ => panic!("Cannot multiply {:?} and {:?}", left, right),
    }
}

fn divide_values(left: &Value, right: &Value) -> Value {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => {
            if *b == 0 {
                panic!("Division by zero");
            }
            Value::Number(a / b)
        }
        _ => panic!("Cannot divide {:?} by {:?}", left, right),
    }
}

fn compare_greater(left: &Value, right: &Value) -> Value {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Value::Boolean(a > b),
        _ => panic!("Cannot compare {:?} > {:?}", left, right),
    }
}

fn compare_greater_equal(left: &Value, right: &Value) -> Value {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Value::Boolean(a >= b),
        _ => panic!("Cannot compare {:?} >= {:?}", left, right),
    }
}

fn compare_less(left: &Value, right: &Value) -> Value {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Value::Boolean(a < b),
        _ => panic!("Cannot compare {:?} < {:?}", left, right),
    }
}

fn compare_less_equal(left: &Value, right: &Value) -> Value {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Value::Boolean(a <= b),
        _ => panic!("Cannot compare {:?} <= {:?}", left, right),
    }
}

fn compare_equal(left: &Value, right: &Value) -> Value {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Value::Boolean(a == b),
        (Value::String(a), Value::String(b)) => Value::Boolean(a == b),
        (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(a == b),
        _ => Value::Boolean(false),
    }
}

fn compare_not_equal(left: &Value, right: &Value) -> Value {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Value::Boolean(a != b),
        (Value::String(a), Value::String(b)) => Value::Boolean(a != b),
        (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(a != b),
        _ => Value::Boolean(true),
    }
}
