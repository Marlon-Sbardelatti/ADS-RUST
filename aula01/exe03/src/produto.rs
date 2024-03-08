#[derive(Debug, Clone)]
pub struct Produto {
    pub name: String,
    pub value: f64,
}

impl<'a> Produto {
    pub fn new(name: String, value: f64) -> Self {
        Self { name, value }
    }
}
