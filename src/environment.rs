use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::expr::LiteralValue;

#[derive(Clone)]
pub struct Environment {
    pub values: HashMap<String, LiteralValue>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &String) -> Option<LiteralValue> {
        let val = self.values.get(name);
        match (val, &self.enclosing) {
            (Some(v), _) => Some(v.clone()),
            (None, Some(t)) => {
                let a = t.borrow_mut().get(name);
                a
            },
            (None, None) => None,
        }
    }

    pub fn assign(&mut self, name: &str, value_ass: LiteralValue) -> bool {
        let old_exist = self.values.contains_key(name);
        match (old_exist, &self.enclosing) {
            (true, _) => {
                self.values.insert(name.to_string(), value_ass).unwrap();
                true
            }
            (false, Some(env)) => {
                env.borrow_mut().assign(name, value_ass);
                true
            }
            _ => false,
        }
    }
}
