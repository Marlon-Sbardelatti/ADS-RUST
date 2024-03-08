#[derive(Debug, Copy, Clone)]
pub struct Produto<'a> {
    name: &'a str,
    value: f64,
}

impl<'a> Produto<'a> {
    pub fn new(name: &'a str, value: f64) -> Self {
        Self { name, value }
    }
}

pub struct Vetor<'a> {
    produtos: *mut Produto<'a>,
    length: usize,
    capacity: usize,
}

impl<'a> Vetor<'a> {
    pub fn new() -> Self {
        let produtos = std::ptr::null_mut();
        Self {
            produtos,
            length: 0,
            capacity: 0,
        }
    }

    pub fn cadastrar(&mut self, produto: Produto<'a>) {
        // Check if there is space in the array
        if self.length < self.capacity {
            // Add the number to the array
            unsafe {
                let index = self.length;
                *self.produtos.add(index) = produto;
                self.length += 1;
            }
        } else {
            // If the array is full, grow it by one element
            self.grow();
            // Add the number to the array
            unsafe {
                let index = self.length;
                *self.produtos.add(index) = produto;
                self.length += 1;
            }
        }
    }

    pub fn listar(&self) {
        // Display the records in the array
        for i in 0..self.length {
            unsafe {
                let value = *self.produtos.add(i);
                println!("{:?}", value);
            }
        }
    }

    fn grow(&mut self) {
        // Allocate a new array with increased capacity
        let new_capacity = if self.capacity == 0 {
            1
        } else {
            self.capacity * 2
        };
        let layout = std::alloc::Layout::array::<Produto<'a>>(new_capacity).unwrap();
        let new_produtos = unsafe { std::alloc::alloc(layout) as *mut Produto<'a> };

        // Copy elements from the old array to the new array
        for i in 0..self.length {
            unsafe {
                *new_produtos.add(i) = std::ptr::read(self.produtos.add(i));
            }
        }

        // Deallocate the old array
        if !self.produtos.is_null() {
            let old_layout = std::alloc::Layout::array::<Produto<'a>>(self.capacity).unwrap();
            unsafe {
                std::alloc::dealloc(self.produtos as *mut u8, old_layout);
            }
        }

        // Update the struct fields with the new array and capacity
        self.produtos = new_produtos;
        self.capacity = new_capacity;
    }
}

impl<'a> Drop for Vetor<'a> {
    fn drop(&mut self) {
        // Deallocate the array when the struct goes out of scope
        if !self.produtos.is_null() {
            let layout = std::alloc::Layout::array::<Produto<'a>>(self.capacity).unwrap();
            unsafe {
                std::alloc::dealloc(self.produtos as *mut u8, layout);
            }
        }
    }
}
