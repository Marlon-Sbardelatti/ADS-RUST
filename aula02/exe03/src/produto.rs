#[derive(Debug)]
pub struct Produto {
    pub name: String,
    pub segmento: String,
}

impl Produto {
    pub fn new(name: String, segmento: String) -> Self {
        Self { name, segmento }
    }
}
