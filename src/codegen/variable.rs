use super::{Identifier, Type};
use indexmap::IndexMap;

#[derive(Debug)]
pub struct VariableMap {
    extern_ids: IndexMap<Identifier, Type>,
    ids: IndexMap<Identifier, Type>,
}

impl VariableMap {
    pub fn empty() -> Self {
        Self {
            extern_ids: IndexMap::new(),
            ids: IndexMap::new(),
        }
    }

    pub fn extend(orig: &VariableMap) -> Self {
        let mut extern_ids = orig.extern_ids.clone();
        for (k, v) in orig.ids.iter() {
            extern_ids.insert(k.clone(), v.clone());
        }


        Self {
            extern_ids,
            ids: IndexMap::new(),
        }
    }

    pub fn declare(&mut self, id: Identifier, t: Type) {
        if self.ids.contains_key(&id) {
            panic!("duplicate variable definition");
        }
        self.ids.insert(id, t);
    }

    pub fn offset_of(&self, id: &Identifier) -> isize {
        let index = if self.ids.contains_key(id) {
            let (i, _, _) = self
                .ids
                .get_full(id)
                .unwrap();
            self.extern_ids.len() + i
        } else {
            let (i, _, _) = self
                .extern_ids
                .get_full(id)
                .expect(&format!("No variable {} in scope", id));
            i
        };

        -8 * (1 + index as isize) // 64 bit offsetting, starting at [rbp-8]
    }
}
