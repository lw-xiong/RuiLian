use std::cell::RefCell;
use std::collections::HashMap;
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
                println!(
                    "Warning: Top-level return value ignored: {:?}",
                    return_value
                );
            }
        }
    }

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
                println!("{}", value_to_string(&value));
                Ok(())
            }
            Stmt::Block(statements) => {
                let new_env = Environment::new_enclosed(&self.environment);
                let previous_env = self.environment.clone();
                self.environment = new_env;

                let mut result = Ok(());
                for stmt in statements {
                    result = self.execute(stmt);
                    if result.is_err() {
                        break;
                    }
                }

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
                        return result;
                    }
                }
                Ok(())
            }
            Stmt::For {
                variable,
                iterable,
                body,
            } => {
                let iterable_value = self.evaluate(iterable);

                match iterable_value {
                    Value::Array(arr) => {
                        for element in arr {
                            let loop_env = Environment::new_enclosed(&self.environment);
                            loop_env.borrow_mut().define(variable.clone(), element);

                            let previous_env = self.environment.clone();
                            self.environment = loop_env;

                            let result = self.execute(body);

                            self.environment = previous_env;

                            if result.is_err() {
                                return result;
                            }
                        }
                        Ok(())
                    }
                    Value::String(s) => {
                        for ch in s.chars() {
                            let loop_env = Environment::new_enclosed(&self.environment);
                            loop_env
                                .borrow_mut()
                                .define(variable.clone(), Value::String(ch.to_string()));

                            let previous_env = self.environment.clone();
                            self.environment = loop_env;

                            let result = self.execute(body);
                            self.environment = previous_env;

                            if result.is_err() {
                                return result;
                            }
                        }
                        Ok(())
                    }
                    _ => panic!("Can only iterate over arrays or strings"),
                }
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
                Err(return_value)
            }
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Number(n) => Value::Number(*n),
            Expr::String(s) => Value::String(s.clone()),
            Expr::Boolean(b) => Value::Boolean(*b),
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
                if let Expr::Variable(name) = callee.as_ref() {
                    match name.as_str() {
                        "print" => {
                            for arg in arguments {
                                let value = self.evaluate(arg);
                                print!("{} ", value_to_string(&value));
                            }
                            println!();
                            return Value::Number(0);
                        }
                        "len" => {
                            if arguments.len() != 1 {
                                panic!("len() expects exactly 1 argument");
                            }
                            let arg_value = self.evaluate(&arguments[0]);
                            match arg_value {
                                Value::String(s) => return Value::Number(s.len() as i64),
                                Value::Array(arr) => return Value::Number(arr.len() as i64),
                                Value::Map(map) => return Value::Number(map.len() as i64),
                                _ => panic!("len() expects a string, array, or map"),
                            }
                        }
                        _ => {}
                    };
                }
                self.call_user_function(callee, arguments)
            }

            Expr::Array(elements) => {
                let array_values = elements.iter().map(|e| self.evaluate(e)).collect();
                Value::Array(array_values)
            }

            Expr::Map(pairs) => {
                let mut map = HashMap::new();
                for (key, value_expr) in pairs {
                    let value = self.evaluate(value_expr);
                    map.insert(key.clone(), value);
                }
                Value::Map(map)
            }

            Expr::Index { object, index } => {
                let object_val = self.evaluate(object);
                let index_val = self.evaluate(index);

                match (object_val, index_val) {
                    (Value::Array(arr), Value::Number(idx)) => {
                        let idx = idx as usize;
                        if idx >= arr.len() {
                            panic!("Array index {} out of bounds", idx);
                        }
                        arr[idx].clone()
                    }
                    (Value::Map(map), Value::String(key)) => map
                        .get(key.as_str())
                        .cloned()
                        .unwrap_or_else(|| Value::Number(0)),
                    (Value::Map(_), index_val) => {
                        panic!("Map key must be a string, got {:?}", index_val)
                    }
                    _ => panic!("Cannot index non-array or non-map"),
                }
            }

            Expr::IndexAssign {
                object,
                index,
                value,
            } => {
                let object_val = self.evaluate(object);
                let index_val = self.evaluate(index);
                let value_val = self.evaluate(value);

                match (object_val, index_val) {
                    (Value::Map(mut map), Value::String(key)) => {
                        map.insert(key, value_val.clone());

                        match object.as_ref() {
                            Expr::Variable(var_name) => {
                                self.environment
                                    .borrow_mut()
                                    .assign(var_name, Value::Map(map.clone()));
                            }
                            _ => {}
                        }
                        value_val
                    }
                    (Value::Array(mut arr), Value::Number(idx)) => {
                        let idx = idx as usize;
                        if idx >= arr.len() {
                            panic!("Array index {} out of bounds", idx);
                        }
                        arr[idx] = value_val.clone();
                        if let Expr::Variable(var_name) = object.as_ref() {
                            self.environment
                                .borrow_mut()
                                .assign(var_name, Value::Array(arr.clone()));
                        }
                        value_val
                    }
                    _ => panic!("Cannot assign to non-array or non-map index"),
                }
            }

            // --- Dot property access ---
            Expr::Dot { object, field } => {
                let object_val = self.evaluate(object);

                match object_val {
                    Value::Map(map) => map
                        .get(field.as_str())
                        .cloned()
                        .unwrap_or_else(|| Value::Number(0)),
                    _ => panic!("Cannot access field '{}' on non-map value", field),
                }
            }

            Expr::DotAssign {
                object,
                field,
                value,
            } => {
                let object_val = self.evaluate(object);
                let value_val = self.evaluate(value);

                match object_val {
                    Value::Map(mut map) => {
                        map.insert(field.clone(), value_val.clone());

                        match object.as_ref() {
                            Expr::Variable(var_name) => {
                                self.environment
                                    .borrow_mut()
                                    .assign(var_name, Value::Map(map.clone()));
                            }
                            _ => {}
                        }
                        value_val
                    }
                    _ => panic!("Cannot assign to field '{}' on non-map value", field),
                }
            }
        }
    }

    fn call_user_function(&mut self, callee: &Expr, arguments: &[Expr]) -> Value {
        let callee_value = self.evaluate(callee);

        match callee_value {
            Value::Function(function) => {
                if arguments.len() != function.params.len() {
                    panic!(
                        "Expected {} arguments but got {}",
                        function.params.len(),
                        arguments.len()
                    );
                }

                let call_env = Environment::new_enclosed(&function.closure);
                let arg_values: Vec<Value> =
                    arguments.iter().map(|arg| self.evaluate(arg)).collect();

                for (param, arg_value) in function.params.iter().zip(arg_values) {
                    call_env.borrow_mut().define(param.clone(), arg_value);
                }

                let previous_env = self.environment.clone();
                self.environment = call_env;

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

                self.environment = previous_env;

                if return_occurred {
                    return_value
                } else {
                    Value::Number(0)
                }
            }
            _ => panic!("Can only call functions"),
        }
    }
}

// ---- Helpers ----
fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Number(n) => *n != 0,
        Value::String(s) => !s.is_empty(),
        Value::Boolean(b) => *b,
        Value::Function(_) => true,
        Value::Array(arr) => !arr.is_empty(),
        Value::Map(map) => !map.is_empty(),
    }
}

fn value_to_string(value: &Value) -> String {
    match value {
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.clone(),
        Value::Boolean(b) => b.to_string(),
        Value::Function(func) => format!("<function {}>", func.name),
        Value::Array(arr) => {
            let elements: Vec<String> = arr.iter().map(|v| value_to_string(v)).collect();
            format!("[{}]", elements.join(", "))
        }
        Value::Map(map) => {
            let mut items = Vec::new();
            for (key, val) in map {
                items.push(format!("{}: {}", key, value_to_string(val)));
            }
            format!("{{{}}}", items.join(", "))
        }
    }
}

fn add_values(left: &Value, right: &Value) -> Value {
    if let Value::String(s) = left {
        return Value::String(s.clone() + &value_to_string(right));
    }
    if let Value::String(s) = right {
        return Value::String(value_to_string(left) + s);
    }

    if let (Value::Array(a), Value::Array(b)) = (left, right) {
        let mut new_array = a.clone();
        new_array.extend(b.clone());
        return Value::Array(new_array);
    }

    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
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
        (Value::Array(a), Value::Array(b)) => Value::Boolean(a == b),
        (Value::Map(a), Value::Map(b)) => Value::Boolean(a == b),
        _ => Value::Boolean(false),
    }
}

fn compare_not_equal(left: &Value, right: &Value) -> Value {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Value::Boolean(a != b),
        (Value::String(a), Value::String(b)) => Value::Boolean(a != b),
        (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(a != b),
        (Value::Array(a), Value::Array(b)) => Value::Boolean(a != b),
        (Value::Map(a), Value::Map(b)) => Value::Boolean(a != b),
        _ => Value::Boolean(true),
    }
}
