mod label;
mod variable;

use std::sync::{Arc, Mutex};
use self::label::LabelGenerator;
use self::variable::VariableMap;
use crate::ast::*;

pub struct Context {
    labels: Arc<Mutex<LabelGenerator>>,
    vars: Arc<Mutex<VariableMap>>,
    outer_loop: Option<(String, String)>,
}

impl Context {
    pub fn empty() -> Self {
        Self {
            labels: Arc::new(Mutex::new(LabelGenerator::new())),
            vars: Arc::new(Mutex::new(VariableMap::empty())),
            outer_loop: None,
        }
    }

    pub fn inner_scope(&mut self) -> Self {
        Self {
            labels: Arc::clone(&mut self.labels),
            vars: Arc::new(Mutex::new(VariableMap::extend(&self.vars.lock().unwrap()))),
            outer_loop: self.outer_loop.clone(),
        }
    }
    pub fn inner_loop(&mut self, outer_loop_cont: String, outer_loop_end: String) -> Self {
        Self {
            labels: self.labels.clone(),
            vars: self.vars.clone(),
            outer_loop: Some((outer_loop_cont, outer_loop_end)),
        }
    }

    pub fn unique_label(&mut self) -> String {
        self.labels.lock().unwrap().unique_label()
    }
    pub fn offset_of(&self, id: &Identifier) -> isize {
        self.vars.lock().unwrap().offset_of(id)
    }
    pub fn declare(&mut self, id: Identifier, t: Type) {
        self.vars.lock().unwrap().declare(id, t)
    }
    pub fn outer_loop(&self) -> Option<&(String, String)> {
        self.outer_loop.as_ref()
    }
}
