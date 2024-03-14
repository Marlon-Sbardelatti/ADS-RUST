#[derive(Debug)]
pub struct User {
    pub name: String,
    pub idade: u32,
}

impl User {
   pub fn new(name: String, idade: u32) -> Self {
      Self{
            name,
            idade
        } 
   } 
}
