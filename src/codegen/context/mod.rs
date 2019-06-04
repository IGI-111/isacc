mod label;
mod variable;

use std::rc::Rc;
use self::label::LabelGenerator;
use self::variable::VariableMap;
use super::{Identifier, Type};

pub struct Context {
    labels: Rc<LabelGenerator>,
    vars: VariableMap,
    outer_loop_end: Option<String>,
}

impl Context {
    pub fn empty() -> Self {
        Self {
            labels: Rc::new(LabelGenerator::new()),
            vars: VariableMap::empty(),
            outer_loop_end: None,
        }
    }

    pub fn inner_scope(&self) -> Self {
        Self {
            labels: self.labels.clone(),
            vars: VariableMap::extend(&self.vars),
            outer_loop_end: self.outer_loop_end.clone(),
        }
    }
    pub fn unique_label(&mut self) -> String {
        Rc::get_mut(&mut self.labels).unwrap().unique_label()
    }
    pub fn offset_of(&self, id: &Identifier) -> isize {
        self.vars.offset_of(id)
    }
    pub fn declare(&mut self, id: Identifier, t: Type) {
        self.vars.declare(id, t)
    }
}
