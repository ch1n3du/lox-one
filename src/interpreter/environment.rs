use std::collections::HashMap;

use crate::lox_literal::LoxLiteral;

#[derive(Debug, Clone)]
pub struct Environment {
    pub enclosing: Option<Box<Environment>>,
    values: HashMap<String, LoxLiteral>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn with_enclosing(enclosing: Box<Environment>) -> Environment {
        Environment {
            enclosing: Some(enclosing),
            values: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<LoxLiteral> {
        let value = self.values.get(name);

        if value.is_some() {
            value.map(|v| v.clone())
        } else {
            match self.enclosing.as_ref() {
                Some(prev) => prev.get(name),
                None => None
            }
        }
    }

    pub fn define(&mut self, name: &str, initializer: LoxLiteral) {
        self.values.insert(name.to_string(), initializer);
    }

    pub fn assign(&mut self, name: &str, value: LoxLiteral) -> Option<()>{
        if self.values.contains_key(name) {
            self.define(name, value);
            return Some(());
        }

        match &mut self.enclosing {
            Some(enclosing) => enclosing.assign(name, value),
            None => None
        }
    }

}
