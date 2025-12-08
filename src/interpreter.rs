use crate::ast::{BinOp, Expr, LogicalOp, Program, Stmt, UnaryOp};
use crate::environment::Environment;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn interpret(&mut self, program: &Program) {
        for stmt in &program.statements {
            self.execute(stmt);
        }
    }

    fn execute(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expr(expr) => {
                self.evaluate(expr);
            }
            Stmt::Let { name, initializer } => {
                let value = match initializer {
                    Some(expr) => self.evaluate(expr),
                    None => 0,
                };
                self.environment.borrow_mut().define(name.clone(), value);
            }
            Stmt::Print(expr) => {
                let value = self.evaluate(expr);
                println!("{}", value);
            }
            Stmt::Block(statements) => {
                // Enter a new block scope
                let new_env = Rc::new(RefCell::new(Environment::new_enclosed(Rc::clone(
                    &self.environment,
                ))));

                // Save the current environment
                let previous_env = Rc::clone(&self.environment);

                // Switch to the new environment
                self.environment = new_env;

                // Execute block statements
                for stmt in statements {
                    self.execute(stmt);
                }

                // Restore previous environment
                self.environment = previous_env;
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition_value = self.evaluate(condition);
                if condition_value != 0 {
                    // Non-zero = true
                    self.execute(then_branch);
                } else if let Some(else_branch) = else_branch {
                    self.execute(else_branch);
                }
            }
            Stmt::While { condition, body } => {
                loop {
                    let condition_value = self.evaluate(condition);
                    if condition_value == 0 {
                        // 0 = false
                        break;
                    }
                    self.execute(body);
                }
            }
        }
    }

    pub fn evaluate(&mut self, expr: &Expr) -> i64 {
        match expr {
            Expr::Number(n) => *n,
            Expr::Boolean(b) => {
                if *b {
                    1
                } else {
                    0
                }
            }
            Expr::Variable(name) => self
                .environment
                .borrow()
                .get(name)
                .unwrap_or_else(|| panic!("Undefined variable '{}'", name)),
            Expr::Assign(name, expr) => {
                let value = self.evaluate(expr);
                if !self.environment.borrow_mut().assign(name, value) {
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
                    BinOp::Add => left_val + right_val,
                    BinOp::Subtract => left_val - right_val,
                    BinOp::Multiply => left_val * right_val,
                    BinOp::Divide => {
                        if right_val == 0 {
                            panic!("Division by zero");
                        }
                        left_val / right_val
                    }
                    BinOp::Greater => (left_val > right_val) as i64,
                    BinOp::GreaterEqual => (left_val >= right_val) as i64,
                    BinOp::Less => (left_val < right_val) as i64,
                    BinOp::LessEqual => (left_val <= right_val) as i64,
                    BinOp::EqualEqual => (left_val == right_val) as i64,
                    BinOp::BangEqual => (left_val != right_val) as i64,
                }
            }
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left_val = self.evaluate(left);

                // Short-circuit evaluation
                match operator {
                    LogicalOp::And => {
                        if left_val == 0 {
                            // false
                            return 0;
                        }
                        self.evaluate(right)
                    }
                    LogicalOp::Or => {
                        if left_val != 0 {
                            // true
                            return 1;
                        }
                        self.evaluate(right)
                    }
                }
            }
            Expr::Unary { operator, right } => {
                let right_val = self.evaluate(right);
                match operator {
                    UnaryOp::Negate => -right_val,
                    UnaryOp::Not => {
                        if right_val == 0 {
                            1
                        } else {
                            0
                        }
                    }
                }
            }
            Expr::Call { callee, arguments } => {
                if callee == "print" {
                    for arg in arguments {
                        let value = self.evaluate(arg);
                        print!("{} ", value);
                    }
                    println!();
                    0
                } else {
                    panic!("Unknown function: {}", callee);
                }
            }
        }
    }
}
