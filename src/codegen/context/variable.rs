use super::{Identifier, Type, super::CALLER_REGS};
use indexmap::IndexMap;

#[derive(Debug)]
pub struct VariableMap {
    args: IndexMap<Identifier, Type>,
    extern_ids: IndexMap<Identifier, Type>,
    ids: IndexMap<Identifier, Type>,
}

impl VariableMap {
    pub fn empty() -> Self {
        Self {
            args: IndexMap::new(),
            extern_ids: IndexMap::new(),
            ids: IndexMap::new(),
        }
    }

    pub fn with_args(args: &Vec<(Type, Identifier)>) -> Self {
        let mut args_map = IndexMap::new();
        for (typename, id) in args {
            args_map.insert(id.clone(), typename.clone());
        }
        Self {
            args: args_map,
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
            args: orig.args.clone(),
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

    pub fn resolve(&self, id: &Identifier) -> String {
        if self.ids.contains_key(id) {
            self.resolve_intern(id)
        } else if self.extern_ids.contains_key(id) {
            self.resolve_extern(id)
        } else if self.args.contains_key(id) {
            self.resolve_arg(id)
        } else {
            panic!(format!("undefined variable: {}; {:?}", id, self));
        }
    }

    fn resolve_intern(&self, id: &Identifier) -> String {
        let (i, _, _) = self.ids.get_full(id).unwrap();
        let index = self.extern_ids.len() + i;

        let offset = -8 * (1 + index as isize); // 64 bit offsetting, starting at [rbp-8]
        format!("QWORD PTR [rbp{}]", offset)
    }
    fn resolve_extern(&self, id: &Identifier) -> String {
        let (index, _, _) = self.extern_ids.get_full(id).unwrap();
        let offset = -8 * (1 + index as isize); // 64 bit offsetting, starting at [rbp-8]
        format!("QWORD PTR [rbp{}]", offset)
    }
    fn resolve_arg(&self, id: &Identifier) -> String {
        let (index, _, _) = self.args.get_full(id).unwrap();
        if index < 6 {
            CALLER_REGS[index].to_string()
        } else {
            let offset = 8 * (2 + (index as isize - 6)); // 64 bit offsetting, starting at [rbp+16]
            format!("QWORD PTR [rbp+{}]", offset)
        }
    }
}
