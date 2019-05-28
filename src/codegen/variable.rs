use super::{Identifier, Type};
use indexmap::IndexMap;

pub struct VariableMap {
    ids: IndexMap<Identifier, Type>,
}

impl VariableMap {
    pub fn new() -> Self {
        Self { ids: IndexMap::new() }
    }

    pub fn declare(&mut self, id: Identifier, t: Type) {
        if self.ids.contains_key(&id) {
            panic!("duplicate variable definition");
        }
        self.ids.insert(id, t);
    }

    pub fn offset_of(&self, id: &Identifier) -> isize {
        let (i, _, _) = self.ids.get_full(id).unwrap();
        -8 * (1 + i as isize) // 64 bit offsetting, starting at [rbp-8]
    }
}
