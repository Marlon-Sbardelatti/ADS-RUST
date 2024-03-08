use crate::Produto;
use text_io::*;

pub struct Vetor {
    produtos: *mut Produto,
    length: usize,
    capacity: usize,
}

impl Vetor {
    pub fn new() -> Self {
        let produtos = std::ptr::null_mut();
        Self {
            produtos,
            length: 0,
            capacity: 0,
        }
    }

    pub fn cadastrar(&mut self, produto: Produto) {
        //verifica o tamanho
        if self.length < self.capacity {
            unsafe {
                let index = self.length;
                *self.produtos.add(index) = produto;
                self.length += 1;
            }
        } else {
            //grow se n houver espaco
            self.grow();
            unsafe {
                let index = self.length;
                *self.produtos.add(index) = produto;
                self.length += 1;
            }
        }
    }

    pub fn listar(&self) {
        println!("\n");
        for i in 0..self.length {
            unsafe {
                let value = &*self.produtos.add(i);
                println!("{:?}", value);
            }
        }
    }

    pub fn pesquisar(&mut self, name: String) {
        let mut found = false;
        for i in 0..self.length {
            unsafe {
                if (*self.produtos.add(i)).name == name {
                    println!("{:?}", &*self.produtos.add(i));
                    found = true;
                }
            }
        }

        if !found {
            println!("Produto não encontrado.")
        }
    }

    pub fn alterar(&mut self, name: String) {
        let mut found = false;
        for i in 0..self.length {
            unsafe {
                if (*self.produtos.add(i)).name == name {
                    println!("Novo nome: ");
                    let novo_nome: String = read!();
                    println!("Novo valor: ");
                    let novo_valor: f64 = read!();
                    *self.produtos.add(i) = Produto::new(novo_nome, novo_valor);

                    found = true;
                }
            }
        }

        if !found {
            println!("Produto não encontrado.")
        }
    }

    pub fn remover(&mut self, name: String) {
        let mut temp_produtos = Vec::with_capacity(self.capacity);

        let mut found = false;
        for i in 0..self.length {
            unsafe {
                if (*self.produtos.add(i)).name == name {
                    found = true;
                } else {
                    temp_produtos.push(std::ptr::read(self.produtos.add(i)));
                }
            }
        }

        //vetor.len recebe a len do temp
        self.length = temp_produtos.len();
        //vetor recebe um ptr de temp
        self.produtos = temp_produtos.as_mut_ptr();
        std::mem::forget(temp_produtos);

        if !found {
            println!("Produto não encontrado.");
        }
    }

    fn grow(&mut self) {
        //aumenta a capacity por 2
        let new_capacity = if self.capacity == 0 {
            1
        } else {
            self.capacity * 2
        };
        //aloca com a nova capacity
        let layout = std::alloc::Layout::array::<Produto>(new_capacity).unwrap();
        let new_produtos = unsafe { std::alloc::alloc(layout) as *mut Produto };

        //copia para o temp
        for i in 0..self.length {
            unsafe {
                *new_produtos.add(i) = std::ptr::read(self.produtos.add(i));
            }
        }

        //dealloc o antigo
        if !self.produtos.is_null() {
            let old_layout = std::alloc::Layout::array::<Produto>(self.capacity).unwrap();
            unsafe {
                std::alloc::dealloc(self.produtos as *mut u8, old_layout);
            }
        }

        //move o temp para o self(vetor)
        self.produtos = new_produtos;
        self.capacity = new_capacity;
    }
}

impl<'a> Drop for Vetor {
    fn drop(&mut self) {
        if !self.produtos.is_null() {
            let layout = std::alloc::Layout::array::<Produto>(self.capacity).unwrap();
            unsafe {
                std::alloc::dealloc(self.produtos as *mut u8, layout);
            }
        }
    }
}
