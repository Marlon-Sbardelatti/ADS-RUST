#[derive(Debug)]
pub struct Aluno {
    name: String,
    idade: u32,
}

impl Aluno {
    pub fn new(name: String, idade: u32) -> Self {
        Self { name, idade }
    }
}
