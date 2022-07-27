use std::cell::RefCell;
use std::collections::HashMap;

use std::rc::Rc;

use crate::lox_literal::LoxLiteral;

pub struct Enviroment {
    pub enclosing: Option<Rc<RefCell<Enviroment>>>,
    values: HashMap<String, LoxLiteral>,
}

impl Enviroment {
    pub fn new() -> Enviroment {
        Enviroment {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn with_enclosing(enclosing: &Rc<RefCell<Enviroment>>) -> Rc<RefCell<Enviroment>> {
        let env = Enviroment {
            enclosing: Some(Rc::clone(enclosing)),
            values: HashMap::new(),
        };

        Rc::new(RefCell::new(env))
    }

    pub fn define(&mut self, name: &String, initializer: LoxLiteral) {
        self.values.insert(name.to_owned(), initializer);
    }

    pub fn get(&self, name: &String) -> Option<LoxLiteral> {
        let val = self.values.get(name);

        match (val, &self.enclosing) {
            (Some(res), _) => Some(res.clone()),
            (_, Some(env)) => env.borrow().get(name).clone(),
            (_, None) => None,
        }
    }
}
