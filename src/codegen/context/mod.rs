mod label;
mod variable;

use std::sync::{Arc, Mutex};
use self::label::LabelGenerator;
use self::variable::VariableMap;
use crate::ast::*;

#[derive(Debug)]
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

    pub fn function_scope(&mut self, fun: &Function) -> Self {
        Self {
            labels: Arc::clone(&mut self.labels),
            vars: Arc::new(Mutex::new(VariableMap::with_args(&fun.args))),
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
    pub fn resolve(&self, id: &Identifier) -> String {
        self.vars.lock().unwrap().resolve(id)
    }
    pub fn declare(&mut self, id: Identifier, t: Type) {
        self.vars.lock().unwrap().declare(id, t)
    }
    pub fn outer_loop(&self) -> Option<&(String, String)> {
        self.outer_loop.as_ref()
    }
}
