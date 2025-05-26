use std::{collections::HashMap, rc::Rc};

use crate::expr::LiteralValue;

#[derive(Clone)]
pub struct Environment {
    pub values: HashMap<String, LiteralValue>,
    pub enclosing: Option<Rc<Environment>>,
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

    pub fn get(&self, name: &String) -> Option<&LiteralValue> {
        let val = self.values.get(name);
        match (val, &self.enclosing) {
            (Some(v), _) => Some(v),
            (None, Some(t)) => t.get(name),
            (None, None) => None,
        }
    }

    pub fn assign(&mut self, name: &str, value_ass: LiteralValue) -> bool {
        let old_exist = self.values.contains_key(name);
        match (old_exist, &mut self.enclosing) {
            (true, _) => {
                self.values.insert(name.to_string(), value_ass).unwrap();
                true
            }
            (false, Some(env)) => {
                Rc::get_mut(env)
                    .expect("cannot get mutable reff")
                    .assign(name, value_ass);
                true
            }
            _ => false,
        }
    }
}
