#[derive(Debug)]
pub struct LabelGenerator {
    next_label: usize,
}

impl LabelGenerator {
    pub fn new() -> Self {
        Self { next_label: 0 }
    }

    pub fn unique_label(&mut self) -> String {
        let res = self.next_label;
        self.next_label += 1;
        format!("_{}", res)
    }
}
