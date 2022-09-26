use std::collections::HashMap;

use crate::lox_value::LoxValue;

#[derive(Debug, Clone)]
pub struct Environment {
    scopes: Vec<HashMap<String, LoxValue>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new())
    }

    pub fn end_scope(&mut self) {
        if self.scopes.len() < 2 {
            panic!("You can't end the global scope dumbass")
        }
        self.scopes.pop();
    }

    pub fn get(&self, name: &str) -> Option<LoxValue> {
        for scope in self.scopes.iter().rev() {
            if scope.contains_key(name) {
                return scope.get(name).map(|v| v.to_owned());
            }
        }

        None
    }

    pub fn get_at(&self, name: &str, depth: usize) -> Option<LoxValue> {
        self.scopes[depth].get(name).map(|v| v.to_owned())
    }

    pub fn set_at(&mut self, name: &str, value: LoxValue, depth: usize) -> Option<()> {
        if self.scopes[depth].contains_key(name) {
            self.scopes[depth].insert(name.to_string(), value);
            Some(())
        } else {
            None
        }
    }

    pub fn define(&mut self, name: &str, initializer: LoxValue) {
        self.scopes
            .last_mut()
            .unwrap()
            .insert(name.to_string(), initializer);
    }

    pub fn assign(&mut self, name: &str, value: LoxValue) -> Option<()> {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), value);
                return Some(());
            }
        }

        None
    }
}
