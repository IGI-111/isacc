use super::Identifier;
use indexmap::IndexSet;

pub struct VariableMap {
    ids: IndexSet<Identifier>,
}

impl VariableMap {
    pub fn new() -> Self {
        Self { ids: Vec::new() }
    }

    pub fn declare(&mut self, id: &Identifier) {
        if self.ids.contains(id) {
            panic!("duplicate variable definition");
        }
        self.ids.insert(id);
    }

    pub fn offset_of(&self, id: &Identifier) {
        let (i, _) = self.ids.get_full(id).unwrap();
        -8 * (1 + i) // 64 bit offsetting, starting at [rbp-8]
    }
}
